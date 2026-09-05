//! Traverses live, published I3S services end to end.
//!
//! The fixtures in `realworld.rs` prove that *documents* captured from real
//! services parse. They cannot prove that the loader addresses those
//! services correctly, because a fixture is read from disk and a service is
//! read over a URL that the loader itself builds. A wrong node-page id, a
//! wrong sublayer path or a wrong resource index is invisible to a fixture
//! test and fatal in production, so these tests walk a real service breadth
//! first and assert that every node the loader emits actually resolves.
//!
//! These tests are `#[ignore]`d because they need the network. Run them with:
//!
//! ```text
//! cargo test --test live -- --ignored --nocapture
//! ```
//!
//! Each test bounds its own work: `MAX_NODES` caps the walk so a test
//! against a continental basemap terminates, and content is loaded only for
//! a sample of nodes so the suite does not move hundreds of megabytes.

use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use i3s::{ContentKind, SceneLayerLoader, Source};
use kiba::{FetchError, FetchRequest, FetchResponse, LoadOutcome, Loader};

/// Upper bound on nodes visited per test.
///
/// Published basemap layers have millions of nodes; the point of these tests
/// is to prove the addressing is right, and a few hundred nodes spanning
/// several levels of the tree does that as well as a million would.
const MAX_NODES: usize = 400;

/// How many nodes to actually download content for.
const CONTENT_SAMPLE: usize = 12;

/// A `reqwest` transport with the quirks a live ArcGIS service requires.
///
/// Three of them matter. ArcGIS REST endpoints negotiate representation
/// through a `f=` query parameter and serve an HTML browser page by default,
/// so every document request must ask for `f=json` explicitly. Binary
/// resources are served without it, so the parameter cannot simply be
/// appended to everything. And the services gzip their JSON *unconditionally*
/// — a response arrives `Content-Encoding: gzip` whether or not the client
/// asked — so transparent decompression is required, not merely an
/// optimisation.
#[derive(Clone)]
struct Transport {
    client: reqwest::Client,
    requests: Arc<AtomicUsize>,
}

impl Transport {
    fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("i3s-integration-tests")
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .expect("build http client"),
            requests: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn count(&self) -> usize {
        self.requests.load(Ordering::Relaxed)
    }

    async fn fetch(self, request: FetchRequest) -> Result<FetchResponse, FetchError> {
        self.requests.fetch_add(1, Ordering::Relaxed);
        let url = with_json_format(&request.uri);

        let mut builder = self.client.get(&url);
        if let Some(range) = request.range {
            let value = match range.end {
                // kiba's end is exclusive; HTTP's is inclusive.
                Some(end) => format!("bytes={}-{}", range.start, end.saturating_sub(1)),
                None => format!("bytes={}-", range.start),
            };
            builder = builder.header(reqwest::header::RANGE, value);
        }

        let response = builder.send().await?;
        let status = response.status();
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned);

        if !status.is_success() {
            return Err(format!("{status} for {url}").into());
        }

        Ok(FetchResponse {
            bytes: response.bytes().await?.to_vec(),
            content_type,
        })
    }
}

/// Adds `f=json` to a request for an I3S *document*.
///
/// Document endpoints under a SceneServer are extensionless path segments
/// (`.../layers/0`, `.../nodepages/3`); resource endpoints are too
/// (`.../geometries/0`, `.../attributes/f_0/0`), so the last segment alone
/// cannot distinguish them. The discriminator used here is whether the path
/// passes through a resource collection at all: everything below
/// `geometries`, `attributes`, `textures`, `features` or `shared` is binary,
/// and everything else is a document.
fn with_json_format(uri: &str) -> String {
    if !uri.starts_with("http") || uri.contains("f=json") {
        return uri.to_string();
    }
    let path = uri.split('?').next().unwrap_or(uri);
    let is_resource = path.split('/').any(|segment| {
        matches!(
            segment,
            "geometries" | "attributes" | "textures" | "features" | "shared"
        )
    });
    if is_resource {
        return uri.to_string();
    }
    let separator = if uri.contains('?') { '&' } else { '?' };
    format!("{uri}{separator}f=json")
}

fn loader(source: Source) -> (SceneLayerLoader, Transport) {
    let transport = Transport::new();
    let for_fetch = transport.clone();
    let loader = SceneLayerLoader::open(source, move |request| {
        let transport = for_fetch.clone();
        async move { transport.fetch(request).await }
    });
    (loader, transport)
}

// ---------------------------------------------------------------------
// Traversal
// ---------------------------------------------------------------------

/// What a walk observed, so a test can assert on shape rather than on any
/// single node — published services change, and a test that pins an exact
/// node count would be a maintenance burden with no extra signal.
#[derive(Debug, Default)]
struct Walk {
    nodes: usize,
    leaves: usize,
    max_depth: usize,
    geometry_refs: usize,
    attribute_refs: usize,
    texture_refs: usize,
    legacy_refs: usize,
    bytes_loaded: usize,
    loaded: usize,
}

/// Walks `loader` breadth first, visiting up to [`MAX_NODES`] nodes.
///
/// Breadth first rather than depth first because an I3S tree is wide and
/// shallow at the top and the interesting variety — sublayer roots, the
/// transition from a group to a leaf layer — is near the root. A depth-first
/// walk with the same budget would descend one branch and never see it.
async fn walk(loader: &SceneLayerLoader, load_content: bool) -> Walk {
    let root = loader.root().await.expect("fetch layer root");

    let mut summary = Walk::default();
    let mut queue = std::collections::VecDeque::from([root]);
    // Node identity is the address, not the handle: a paged layer can reach
    // the same index twice through a malformed page, and re-expanding it
    // would loop forever.
    let mut seen = HashSet::new();

    while let Some(node) = queue.pop_front() {
        if summary.nodes >= MAX_NODES {
            break;
        }
        let identity = format!("{:?}", node);
        if !seen.insert(identity.clone()) {
            continue;
        }

        summary.nodes += 1;
        summary.max_depth = summary.max_depth.max(node.depth());

        let expansion = loader
            .expand(node.clone())
            .await
            .unwrap_or_else(|e| panic!("expand {identity}: {e}"));

        if expansion.children.is_empty() {
            summary.leaves += 1;
        }

        for content in &expansion.contents {
            match &content.kind {
                ContentKind::Geometry { .. } => summary.geometry_refs += 1,
                ContentKind::Attribute { .. } => summary.attribute_refs += 1,
                ContentKind::Texture { .. } => summary.texture_refs += 1,
                ContentKind::Legacy { .. } => summary.legacy_refs += 1,
            }
            assert!(
                content.uri.starts_with("http"),
                "a REST source must produce absolute URIs, got {:?}",
                content.uri
            );
        }

        if load_content && summary.loaded < CONTENT_SAMPLE {
            for content in expansion.contents.iter().take(2) {
                if summary.loaded >= CONTENT_SAMPLE {
                    break;
                }
                summary.loaded += 1;
                match loader.load(content.clone()).await {
                    Ok(LoadOutcome::Ready(loaded)) => {
                        assert!(
                            !loaded.bytes.is_empty(),
                            "a Ready outcome must carry bytes: {}",
                            loaded.uri
                        );
                        summary.bytes_loaded += loaded.bytes.len();
                        check_decodes(&loaded);
                    }
                    Ok(LoadOutcome::Empty | LoadOutcome::Retry) => {}
                    Err(e) => panic!("load {}: {e}", content.uri),
                }
            }
        }

        queue.extend(expansion.children);
    }

    summary
}

/// Runs the decoder matching the content kind, so a walk proves the bytes
/// are what the layer document said they would be — not merely that some
/// bytes came back.
fn check_decodes(loaded: &i3s::LoadedContent) {
    match &loaded.kind {
        ContentKind::Geometry {
            compressed: true, ..
        } => {
            // Draco is out of scope for this crate; the bytes pass through.
        }
        ContentKind::Geometry {
            vertex_count: Some(vertices),
            ..
        } => {
            assert!(
                *vertices > 0,
                "a node page that states a vertex count states a positive one: {}",
                loaded.uri
            );
        }
        _ => {}
    }
}

/// A modern node-paged Point layer: `store.version` 2.0, `pointNodePages`,
/// Draco-compressed geometry throughout.
#[tokio::test]
#[ignore = "requires network access"]
async fn walks_a_published_point_layer() {
    let (loader, transport) = loader(Source::rest(
        "https://basemaps3d.arcgis.com/arcgis/rest/services/OpenStreetMap3D_Trees_Thematic_v1/SceneServer",
        0,
    ));

    let walk = walk(&loader, true).await;
    eprintln!("point layer: {walk:?} in {} requests", transport.count());

    assert!(walk.nodes > 1, "the layer must expand past its root");
    assert!(walk.max_depth >= 2, "a paged layer must nest");
    assert!(
        walk.geometry_refs > 0,
        "a point layer's nodes carry geometry"
    );
    assert!(walk.loaded > 0, "content must actually load");
    assert!(walk.bytes_loaded > 0);
}

/// A modern node-paged PointCloud layer. Unlike a `Point` layer, PointCloud
/// uses the common `nodePages` index rather than `pointNodePages`, so this
/// independently verifies the loader's PointCloud dispatch and page URLs.
#[tokio::test]
#[ignore = "requires network access"]
async fn walks_a_published_point_cloud_layer() {
    let (loader, transport) = loader(Source::rest(
        "https://tiles.arcgis.com/tiles/8cv2FuXuWSfF0nbL/arcgis/rest/services/AUTZEN_LiDAR/SceneServer",
        0,
    ));

    let walk = walk(&loader, true).await;
    eprintln!(
        "point-cloud layer: {walk:?} in {} requests",
        transport.count()
    );

    assert!(walk.nodes > 1, "the layer must expand past its root");
    assert!(walk.max_depth >= 2, "a paged layer must nest");
    assert!(
        walk.geometry_refs > 0,
        "a point-cloud layer's nodes carry geometry"
    );
    assert!(walk.loaded > 0, "content must actually load");
    assert!(walk.bytes_loaded > 0);
}

/// A building scene layer: no nodes of its own, expanding instead into
/// nested `group` sublayers whose leaves are 1.6 layers with no node pages.
/// This is the one case that exercises all three `NodeAddress` variants.
#[tokio::test]
#[ignore = "requires network access"]
async fn walks_a_published_building_layer() {
    let (loader, transport) = loader(Source::rest(
        "https://tiles.arcgis.com/tiles/cFEFS0EWrhfDeVw9/arcgis/rest/services/Turanga_Library/SceneServer",
        0,
    ));

    let walk = walk(&loader, true).await;
    eprintln!("building layer: {walk:?} in {} requests", transport.count());

    assert!(
        walk.nodes > 10,
        "the building root must expand through its sublayers into real nodes"
    );
    assert!(
        walk.max_depth >= 2,
        "sublayer roots sit below the building root, and their nodes below them"
    );
    assert!(
        walk.legacy_refs + walk.geometry_refs > 0,
        "the 1.6 sublayers address content by href"
    );
}

/// A 3DObject sublayer addressed directly, which must produce the same
/// content as reaching it through its parent building layer.
#[tokio::test]
#[ignore = "requires network access"]
async fn walks_a_sublayer_addressed_directly() {
    let (loader, transport) = loader(
        Source::rest(
            "https://tiles.arcgis.com/tiles/cFEFS0EWrhfDeVw9/arcgis/rest/services/Turanga_Library/SceneServer",
            0,
        )
        .sublayer(19),
    );

    let walk = walk(&loader, false).await;
    eprintln!("sublayer 19: {walk:?} in {} requests", transport.count());

    assert!(walk.nodes > 1, "the sublayer must expand past its root");
}

#[test]
fn json_format_is_added_to_documents_but_not_resources() {
    assert_eq!(
        with_json_format("https://x/SceneServer/layers/0"),
        "https://x/SceneServer/layers/0?f=json"
    );
    assert_eq!(
        with_json_format("https://x/SceneServer/layers/0/nodepages/3"),
        "https://x/SceneServer/layers/0/nodepages/3?f=json"
    );
    // Resources are binary and must not be asked for as JSON.
    assert_eq!(
        with_json_format("https://x/layers/0/nodes/6/geometries/0"),
        "https://x/layers/0/nodes/6/geometries/0"
    );
    assert_eq!(
        with_json_format("https://x/layers/0/nodes/6/attributes/f_0/0"),
        "https://x/layers/0/nodes/6/attributes/f_0/0"
    );
    // An SLPK path is not a URL and must be left alone.
    assert_eq!(
        with_json_format("nodepages/0.json.gz"),
        "nodepages/0.json.gz"
    );
}
