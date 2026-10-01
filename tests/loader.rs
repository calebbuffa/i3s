//! End-to-end traversal of synthetic scene layers through the
//! [`hiera::Loader`] protocol.
//!
//! These exercise the loader against complete, if tiny, layer documents so
//! that URL construction, page caching, profile selection and content
//! enumeration are checked together rather than in isolation.

use std::{
    collections::HashMap,
    future::Future,
    sync::{Arc, Mutex},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

use hiera::{FetchResponse, LoadOutcome, Loader};
use i3s::{ContentKind, SceneLayerLoader, SceneLayerReader, Source};

/// A future that is always immediately ready, so the tests need no
/// executor beyond [`block_on`].
mod ready {
    use std::{future::Future, pin::Pin, task::Poll};

    pub struct Ready(Option<Result<hiera::FetchResponse, hiera::FetchError>>);

    impl Ready {
        pub fn ok(response: hiera::FetchResponse) -> Self {
            Self(Some(Ok(response)))
        }

        pub fn err(message: String) -> Self {
            Self(Some(Err(message.into())))
        }
    }

    impl Future for Ready {
        type Output = Result<hiera::FetchResponse, hiera::FetchError>;

        fn poll(mut self: Pin<&mut Self>, _: &mut std::task::Context<'_>) -> Poll<Self::Output> {
            Poll::Ready(self.0.take().expect("polled after completion"))
        }
    }
}

/// Drives a future to completion on the calling thread.
///
/// The loader is runtime-agnostic, and pulling in an async runtime purely
/// to await a handful of futures would test the runtime rather than the
/// loader. Every future here completes without yielding because the fetch
/// callback is synchronous.
fn block_on<F: Future>(future: F) -> F::Output {
    fn noop_raw_waker() -> RawWaker {
        fn noop(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            noop_raw_waker()
        }
        RawWaker::new(
            std::ptr::null(),
            &RawWakerVTable::new(clone, noop, noop, noop),
        )
    }
    // SAFETY: the vtable's functions are all no-ops over a null pointer.
    let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
    let mut context = Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => std::hint::spin_loop(),
        }
    }
}

/// A fetch callback backed by a fixed map, recording requests so that
/// caching can be asserted rather than assumed.
#[derive(Clone, Default)]
struct Registry {
    files: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    requests: Arc<Mutex<Vec<String>>>,
}

impl Registry {
    fn with(entries: Vec<(&str, String)>) -> Self {
        let registry = Self::default();
        for (uri, body) in entries {
            registry
                .files
                .lock()
                .unwrap()
                .insert(uri.into(), body.into_bytes());
        }
        registry
    }

    fn into_fetch(self) -> impl Fn(hiera::FetchRequest) -> ready::Ready + Send + Sync + 'static {
        move |request| {
            self.requests.lock().unwrap().push(request.uri.clone());
            match self.files.lock().unwrap().get(&request.uri).cloned() {
                Some(bytes) => ready::Ready::ok(FetchResponse {
                    bytes: bytes.into(),
                    content_type: None,
                }),
                None => ready::Ready::err(format!("no such resource: {}", request.uri)),
            }
        }
    }

    fn request_count(&self, uri: &str) -> usize {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|seen| *seen == uri)
            .count()
    }
}

/// Serializes a layer document built from the generated types.
///
/// Hand-writing the JSON would mean hand-maintaining every field the
/// specification marks required (`store.defaultGeometrySchema` and its own
/// required members, and so on). Building the value from the generated
/// structs and serializing it keeps the fixtures valid by construction,
/// and makes them fail to compile rather than fail at runtime if the
/// schema changes.
fn to_json<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).expect("fixture serializes")
}

fn mesh_layer() -> String {
    use i3s::cmn::*;

    let layer = SceneLayerInfo {
        id: 0,
        layer_type: LayerType::ThreeDObject,
        version: "1.7".into(),
        node_pages: Some(NodePageDefinition {
            nodes_per_page: 2,
            root_index: Some(0),
            ..Default::default()
        }),
        geometry_definitions: vec![GeometryDefinition {
            topology: Some(GeometryDefinitionTopology::Triangle),
            geometry_buffers: vec![
                GeometryBuffer {
                    offset: Some(8),
                    position: Some(GeometryPosition::default()),
                    ..Default::default()
                },
                GeometryBuffer {
                    compressed_attributes: Some(CompressedAttributes {
                        encoding: CompressedAttributesEncoding::Draco,
                        attributes: vec![Attributes::Position],
                    }),
                    ..Default::default()
                },
            ],
        }],
        // Keys are deliberately non-contiguous: `f_3` must be addressed by
        // its key, not by its position in this array.
        attribute_storage_info: vec![
            AttributeStorageInfo {
                key: "f_0".into(),
                name: "OBJECTID".into(),
                ..Default::default()
            },
            AttributeStorageInfo {
                key: "f_3".into(),
                name: "Height".into(),
                ..Default::default()
            },
        ],
        texture_set_definitions: vec![TextureSetDefinition {
            formats: vec![
                TextureSetDefinitionFormat {
                    name: "0".into(),
                    format: Format::Jpg,
                },
                TextureSetDefinitionFormat {
                    name: "0_0_1".into(),
                    format: Format::Dds,
                },
            ],
            ..Default::default()
        }],
        ..Default::default()
    };
    to_json(&layer)
}

fn mesh_page_0() -> String {
    use i3s::cmn::*;

    let obb = Obb {
        center: [0.0, 0.0, 0.0],
        half_size: [1.5, 1.5, 1.5],
        quaternion: [0.0, 0.0, 0.0, 1.0],
    };
    let page = NodePage {
        nodes: vec![
            Node {
                index: 0,
                children: vec![1],
                obb: obb.clone(),
                ..Default::default()
            },
            Node {
                index: 1,
                obb,
                mesh: Some(Mesh {
                    geometry: Some(MeshGeometry {
                        definition: 0,
                        // Deliberately not the node index: a node's
                        // payload is addressed by `resource`.
                        resource: 7,
                        vertex_count: Some(36),
                        feature_count: Some(2),
                    }),
                    attribute: Some(MeshAttribute { resource: 7 }),
                    material: Some(MeshMaterial {
                        definition: 0,
                        resource: Some(7),
                        ..Default::default()
                    }),
                }),
                ..Default::default()
            },
        ],
    };
    to_json(&page)
}

fn rest_registry() -> Registry {
    Registry::with(vec![
        ("https://example.com/SceneServer/layers/0", mesh_layer()),
        (
            "https://example.com/SceneServer/layers/0/nodepages/0",
            mesh_page_0(),
        ),
        (
            "https://example.com/SceneServer/layers/0/nodes/7/geometries/0",
            "geometry-bytes".into(),
        ),
    ])
}

fn rest_loader(registry: &Registry) -> SceneLayerLoader {
    SceneLayerLoader::open(
        Source::rest("https://example.com/SceneServer", 0),
        registry.clone().into_fetch(),
    )
}

#[test]
fn traverses_a_node_paged_rest_layer() {
    let registry = rest_registry();
    let loader = rest_loader(&registry);

    let root = block_on(loader.root()).expect("root");
    assert_eq!(root.index(), Some(0));
    assert_eq!(root.depth(), 0);

    let expansion = block_on(loader.expand(root)).expect("expand root");
    // The root node carries no mesh, so it contributes only a child.
    assert!(expansion.contents.is_empty());
    assert_eq!(expansion.children.len(), 1);

    let child = expansion.children[0].clone();
    assert_eq!(child.index(), Some(1));
    assert_eq!(child.depth(), 1);

    let leaf = block_on(loader.expand(child)).expect("expand child");
    assert!(leaf.children.is_empty());

    let uris: Vec<&str> = leaf.contents.iter().map(|c| c.uri.as_str()).collect();
    assert_eq!(
        uris,
        vec![
            "https://example.com/SceneServer/layers/0/nodes/7/geometries/0",
            "https://example.com/SceneServer/layers/0/nodes/7/attributes/f_0/0",
            "https://example.com/SceneServer/layers/0/nodes/7/attributes/f_3/0",
            "https://example.com/SceneServer/layers/0/nodes/7/textures/0",
            "https://example.com/SceneServer/layers/0/nodes/7/textures/0_0_1",
        ]
    );

    // The uncompressed buffer must be chosen by default, and the counts
    // declared by the node page must survive onto the reference.
    match &leaf.contents[0].kind {
        ContentKind::Geometry {
            definition,
            buffer,
            compressed,
            vertex_count,
            feature_count,
        } => {
            assert_eq!((*definition, *buffer), (0, 0));
            assert!(!compressed);
            assert_eq!(*vertex_count, Some(36));
            assert_eq!(*feature_count, Some(2));
        }
        other => panic!("expected geometry, got {other:?}"),
    }

    // Attributes are addressed by their `f_N` key, not their position:
    // the second attribute is `f_3`, so a positional URL would be wrong.
    match &leaf.contents[2].kind {
        ContentKind::Attribute { key, name } => {
            assert_eq!(key, "f_3");
            assert_eq!(name, "Height");
        }
        other => panic!("expected attribute, got {other:?}"),
    }
}

#[test]
fn fetches_each_node_page_once() {
    let registry = rest_registry();
    let loader = Arc::new(rest_loader(&registry));

    let root = block_on(loader.root()).expect("root");
    let expansion = block_on(loader.expand(root.clone())).expect("expand root");
    // Both the root and its child live in page 0, so expanding the child
    // must reuse the retained page rather than re-fetch it.
    block_on(loader.expand(expansion.children[0].clone())).expect("expand child");
    block_on(loader.expand(root)).expect("re-expand root");

    assert_eq!(
        registry.request_count("https://example.com/SceneServer/layers/0/nodepages/0"),
        1
    );
    assert_eq!(
        registry.request_count("https://example.com/SceneServer/layers/0"),
        1
    );
}

#[test]
fn reader_composes_with_a_shared_loader() {
    let registry = rest_registry();
    let loader = Arc::new(rest_loader(&registry));

    let root = block_on(loader.root()).expect("traverse root");
    assert_eq!(root.index(), Some(0));

    let reader = SceneLayerReader::new(Arc::clone(&loader));
    let document = block_on(reader.layer()).expect("typed layer read");
    assert!(matches!(document, i3s::SceneLayerDocument::Mesh(_)));
    assert_eq!(
        registry.request_count("https://example.com/SceneServer/layers/0"),
        2,
        "the loader and reader use the same configured transport"
    );
}

#[test]
fn reader_loads_content_emitted_by_the_loader() {
    let registry = rest_registry();
    let loader = Arc::new(rest_loader(&registry));
    let root = block_on(loader.root()).expect("root");
    let child = block_on(loader.expand(root))
        .expect("expand root")
        .children
        .pop()
        .expect("child");
    let content = block_on(loader.expand(child))
        .expect("expand child")
        .contents
        .into_iter()
        .next()
        .expect("geometry content");

    let reader = SceneLayerReader::new(loader);
    let loaded = block_on(reader.load(content)).expect("load content");
    let LoadOutcome::Ready(loaded) = loaded else {
        panic!("expected loaded content");
    };
    assert_eq!(&loaded.bytes[..], b"geometry-bytes");
}

#[test]
fn describes_a_node_with_cached_native_selection_facts() {
    let registry = rest_registry();
    let loader = rest_loader(&registry);

    let root = block_on(loader.root()).expect("root");
    let info = block_on(loader.describe(root)).expect("describe root");

    assert!(matches!(info.bounds, i3s::NodeBounds::OrientedBox(_)));
    assert_eq!(
        registry.request_count("https://example.com/SceneServer/layers/0/nodepages/0"),
        1
    );
}

#[test]
fn loads_content_bytes() {
    let registry = rest_registry();
    let loader = rest_loader(&registry);

    let root = block_on(loader.root()).expect("root");
    let child = block_on(loader.expand(root)).expect("expand").children[0].clone();
    let contents = block_on(loader.expand(child)).expect("expand").contents;

    match block_on(loader.load(contents[0].clone())).expect("load") {
        hiera::LoadOutcome::Ready(loaded) => {
            assert_eq!(loaded.bytes.as_ref(), b"geometry-bytes");
            assert!(matches!(loaded.kind, ContentKind::Geometry { .. }));
        }
        other => panic!("expected ready content, got {other:?}"),
    }
}

#[test]
fn prefers_the_compressed_buffer_on_request() {
    let registry = rest_registry();
    let loader = rest_loader(&registry)
        .with_geometry_preference(i3s::GeometryEncodingPreference::Compressed);

    let root = block_on(loader.root()).expect("root");
    let child = block_on(loader.expand(root)).expect("expand").children[0].clone();
    let contents = block_on(loader.expand(child)).expect("expand").contents;

    assert_eq!(
        contents[0].uri,
        "https://example.com/SceneServer/layers/0/nodes/7/geometries/1"
    );
    assert!(matches!(
        contents[0].kind,
        ContentKind::Geometry {
            compressed: true,
            ..
        }
    ));
}

#[test]
fn names_packaged_resources_with_extensions() {
    // The same layer inside an exploded package: no `/layers/{id}`
    // segment, and every document gains `.json`/`.bin` plus `.gz`.
    let registry = Registry::with(vec![
        ("file:///pkg/3dSceneLayer.json.gz", mesh_layer()),
        ("file:///pkg/nodepages/0.json.gz", mesh_page_0()),
    ]);
    let loader = SceneLayerLoader::open(Source::eslpk("file:///pkg"), registry.into_fetch());

    let root = block_on(loader.root()).expect("root");
    let child = block_on(loader.expand(root)).expect("expand").children[0].clone();
    let expansion = block_on(loader.expand(child)).expect("expand");

    let uris: Vec<&str> = expansion.contents.iter().map(|c| c.uri.as_str()).collect();
    assert_eq!(
        uris,
        vec![
            "file:///pkg/nodes/7/geometries/0.bin.gz",
            "file:///pkg/nodes/7/attributes/f_0/0.bin.gz",
            "file:///pkg/nodes/7/attributes/f_3/0.bin.gz",
            // Textures keep their own extension and are never gzipped.
            "file:///pkg/nodes/7/textures/0.jpg",
            "file:///pkg/nodes/7/textures/0_0_1.bin.dds",
        ]
    );
}

#[test]
fn names_slpk_resources_as_archive_relative_paths() {
    let registry = Registry::with(vec![
        ("3dSceneLayer.json.gz", mesh_layer()),
        ("nodepages/0.json.gz", mesh_page_0()),
    ]);
    let loader = SceneLayerLoader::open(Source::slpk(), registry.into_fetch());

    let root = block_on(loader.root()).expect("root");
    let child = block_on(loader.expand(root)).expect("expand").children[0].clone();
    let expansion = block_on(loader.expand(child)).expect("expand");

    assert_eq!(
        expansion.contents[0].uri, "nodes/7/geometries/0.bin.gz",
        "an slpk entry name has no scheme and no leading slash"
    );
}

#[test]
fn traverses_a_point_cloud_layer() {
    use i3s::cmn::Obb;
    use i3s::pcsl::{
        AttributeInfoPcsl, IndexPcsl, LayerPcsl, LayerType, NodePageDefinitionPcsl, NodePcsl,
        StorePcsl,
    };

    let layer = LayerPcsl {
        id: 0,
        layer_type: LayerType::PointCloud,
        name: "points".into(),
        attribute_storage_info: vec![AttributeInfoPcsl {
            key: "f_0".into(),
            name: "INTENSITY".into(),
            ..Default::default()
        }],
        store: StorePcsl {
            index: IndexPcsl {
                nodes_per_page: 4,
                node_version: 1,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };

    let obb = Obb {
        center: [0.0, 0.0, 0.0],
        half_size: [1.5, 1.5, 1.5],
        quaternion: [0.0, 0.0, 0.0, 1.0],
    };
    // Point-cloud children are a `firstChild`/`childCount` run, so a node
    // with childCount 2 starting at 1 must yield nodes 1 and 2.
    let page = NodePageDefinitionPcsl {
        nodes: vec![
            NodePcsl {
                child_count: 2,
                first_child: 1,
                resource_id: 0,
                obb: obb.clone(),
                ..Default::default()
            },
            NodePcsl {
                child_count: 0,
                first_child: 0,
                resource_id: 1,
                obb: obb.clone(),
                vertex_count: Some(100),
                ..Default::default()
            },
            NodePcsl {
                child_count: 0,
                first_child: 0,
                resource_id: 2,
                obb,
                ..Default::default()
            },
        ],
    };

    let registry = Registry::with(vec![
        ("https://example.com/SceneServer/layers/0", to_json(&layer)),
        (
            "https://example.com/SceneServer/layers/0/nodepages/0",
            to_json(&page),
        ),
    ]);
    let loader = rest_loader(&registry);

    let root = block_on(loader.root()).expect("root");
    let expansion = block_on(loader.expand(root)).expect("expand root");
    let indices: Vec<Option<u64>> = expansion.children.iter().map(|c| c.index()).collect();
    assert_eq!(indices, vec![Some(1), Some(2)]);

    let leaf = block_on(loader.expand(expansion.children[0].clone())).expect("expand leaf");
    assert!(leaf.children.is_empty());
    let uris: Vec<&str> = leaf.contents.iter().map(|c| c.uri.as_str()).collect();
    assert_eq!(
        uris,
        vec![
            "https://example.com/SceneServer/layers/0/nodes/1/geometries/0",
            "https://example.com/SceneServer/layers/0/nodes/1/attributes/f_0/0",
        ]
    );
    assert!(matches!(
        leaf.contents[0].kind,
        ContentKind::Geometry {
            vertex_count: Some(100),
            ..
        }
    ));
}

#[test]
fn traverses_a_point_layer_through_its_point_node_pages() {
    use i3s::cmn::{
        AttributeStorageInfo, Attributes, CompressedAttributes, CompressedAttributesEncoding, Mesh,
        MeshAttribute, MeshGeometry, Node, NodePage, NodePageDefinition, Obb,
    };
    use i3s::psl::{
        GeometryBufferPsl, GeometryDefinitionPsl, LayerType, SceneLayerInfoPsl, Topology,
    };

    let layer = SceneLayerInfoPsl {
        id: 0,
        layer_type: LayerType::Point,
        name: Some("trees".into()),
        version: "1.8".into(),
        // The page index is spelled `pointNodePages`, but it is an
        // ordinary `nodePageDefinition` over `cmn::Node` records. Real
        // services omit `rootIndex`, which must then default to 0.
        point_node_pages: Some(NodePageDefinition {
            nodes_per_page: 2,
            root_index: None,
            ..Default::default()
        }),
        // Plural: the psl property table says `geometryDefinition`, but
        // every published Point service emits the array form.
        geometry_definitions: vec![GeometryDefinitionPsl {
            topology: Some(Topology::Point),
            geometry_buffers: vec![GeometryBufferPsl {
                compressed_attributes: Some(CompressedAttributes {
                    encoding: CompressedAttributesEncoding::Draco,
                    attributes: vec![Attributes::Position, Attributes::FeatureIndex],
                }),
            }],
        }],
        attribute_storage_info: vec![AttributeStorageInfo {
            key: "f_2".into(),
            name: "SPECIES".into(),
            ..Default::default()
        }],
        ..Default::default()
    };

    let obb = Obb {
        center: [0.0, 0.0, 0.0],
        half_size: [1.0, 1.0, 1.0],
        quaternion: [0.0, 0.0, 0.0, 1.0],
    };
    let page = NodePage {
        nodes: vec![
            Node {
                index: 0,
                children: vec![1],
                obb: obb.clone(),
                ..Default::default()
            },
            Node {
                index: 1,
                obb,
                mesh: Some(Mesh {
                    geometry: Some(MeshGeometry {
                        definition: 0,
                        resource: 5,
                        vertex_count: Some(400),
                        ..Default::default()
                    }),
                    attribute: Some(MeshAttribute { resource: 5 }),
                    material: None,
                }),
                ..Default::default()
            },
        ],
    };

    let registry = Registry::with(vec![
        ("https://example.com/SceneServer/layers/0", to_json(&layer)),
        (
            "https://example.com/SceneServer/layers/0/nodepages/0",
            to_json(&page),
        ),
    ]);
    let loader = rest_loader(&registry);

    let root = block_on(loader.root()).expect("root");
    assert!(root.point_layer().is_some());
    let expansion = block_on(loader.expand(root)).expect("expand root");
    assert_eq!(expansion.children.len(), 1);

    let leaf = block_on(loader.expand(expansion.children[0].clone())).expect("expand leaf");
    let uris: Vec<&str> = leaf.contents.iter().map(|c| c.uri.as_str()).collect();
    assert_eq!(
        uris,
        vec![
            "https://example.com/SceneServer/layers/0/nodes/5/geometries/0",
            "https://example.com/SceneServer/layers/0/nodes/5/attributes/f_2/0",
        ]
    );
    // Point geometry is always Draco, so the reference must say so even
    // though nothing was selected between.
    assert!(matches!(
        leaf.contents[0].kind,
        ContentKind::Geometry {
            compressed: true,
            vertex_count: Some(400),
            ..
        }
    ));
}

#[test]
fn descends_a_building_layer_into_its_geometry_sublayers() {
    use i3s::bld::{LayerBld, LayerBldLayerType, SublayerBld, SublayerBldLayerType};

    // `group` sublayers carry no resources of their own and nest, so the
    // only sublayer with content here is reached two levels down.
    let layer = LayerBld {
        // Not 0: a building layer id and a sublayer id are in different
        // namespaces, and clients must not assume either value.
        id: 4,
        layer_type: LayerBldLayerType::Building,
        name: "tower".into(),
        version: "1.8".into(),
        sublayers: vec![SublayerBld {
            id: 1,
            name: "Architectural".into(),
            layer_type: SublayerBldLayerType::Group,
            sublayers: vec![SublayerBld {
                id: 9,
                name: "Walls".into(),
                layer_type: SublayerBldLayerType::ThreeDObject,
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };

    let registry = Registry::with(vec![
        ("https://example.com/SceneServer/layers/4", to_json(&layer)),
        // The sublayer is a complete scene layer of its own, served below
        // `/sublayers/{id}` -- so the ordinary mesh fixture serves it.
        (
            "https://example.com/SceneServer/layers/4/sublayers/9",
            mesh_layer(),
        ),
        (
            "https://example.com/SceneServer/layers/4/sublayers/9/nodepages/0",
            mesh_page_0(),
        ),
    ]);
    let loader = Arc::new(SceneLayerLoader::open(
        Source::rest("https://example.com/SceneServer", 4),
        registry.clone().into_fetch(),
    ));

    let root = block_on(loader.root()).expect("root");
    assert_eq!(root.building().map(|l| &*l.name), Some("tower"));
    // A building layer has no nodes, so its root is neither paged nor a
    // legacy document.
    assert_eq!(root.index(), None);
    assert_eq!(root.uri(), None);

    let expansion = block_on(loader.expand(root)).expect("expand root");
    // The `group` sublayer contributes structure only; the loader yields
    // the root of the one sublayer that actually holds geometry.
    assert!(expansion.contents.is_empty());
    assert_eq!(expansion.children.len(), 1);

    let sublayer_root = expansion.children[0].clone();
    assert_eq!(sublayer_root.index(), Some(0));
    assert_eq!(sublayer_root.depth(), 1);

    // From here, traversal is exactly a mesh layer's -- but every URL is
    // rooted at the sublayer.
    let inner = block_on(loader.expand(sublayer_root)).expect("expand sublayer root");
    let leaf = block_on(loader.expand(inner.children[0].clone())).expect("expand leaf");
    let uris: Vec<&str> = leaf.contents.iter().map(|c| c.uri.as_str()).collect();
    assert_eq!(
        uris[0],
        "https://example.com/SceneServer/layers/4/sublayers/9/nodes/7/geometries/0"
    );
    assert_eq!(
        uris[1],
        "https://example.com/SceneServer/layers/4/sublayers/9/nodes/7/attributes/f_0/0"
    );

    let sublayer = SceneLayerReader::new(Arc::clone(&loader)).sublayer(9);
    assert!(matches!(
        block_on(sublayer.layer()).expect("read typed sublayer"),
        i3s::SceneLayerDocument::Mesh(_)
    ));
    assert_eq!(
        sublayer.source().sublayer_id(),
        Some(9),
        "the typed reader retains the geometry sublayer address"
    );
}

#[test]
fn traverses_a_legacy_node_document_layer() {
    use i3s::cmn::{LayerType, NodeIndexDocument, NodeReference, Resource, SceneLayerInfo};

    // A layer without `nodePages` is a 1.6 layer whose tree is a graph of
    // `3dNodeIndexDocument`s linked by relative hrefs. Its root is named by
    // `store.rootNode`, which real services publish as `./nodes/root` --
    // not as node 0.
    let layer = SceneLayerInfo {
        id: 0,
        layer_type: LayerType::ThreeDObject,
        version: "1.6".into(),
        node_pages: None,
        store: i3s::cmn::Store {
            root_node: Some("./nodes/root".into()),
            ..Default::default()
        },
        ..Default::default()
    };
    let root_node = NodeIndexDocument {
        id: "root".into(),
        mbs: [0.0, 0.0, 0.0, 10.0],
        children: vec![NodeReference {
            id: "1".into(),
            href: Some("./1".into()),
            mbs: [0.0, 0.0, 0.0, 5.0],
            ..Default::default()
        }],
        geometry_data: vec![Resource {
            href: "./geometries/0".into(),
            ..Default::default()
        }],
        attribute_data: vec![Resource {
            href: "./attributes/f_0/0".into(),
            ..Default::default()
        }],
        shared_resource: Some(Resource {
            href: "./shared".into(),
            ..Default::default()
        }),
        ..Default::default()
    };
    let child_node = NodeIndexDocument {
        id: "1".into(),
        mbs: [0.0, 0.0, 0.0, 5.0],
        geometry_data: vec![Resource {
            href: "./geometries/0".into(),
            ..Default::default()
        }],
        ..Default::default()
    };

    let registry = Registry::with(vec![
        ("https://example.com/SceneServer/layers/0", to_json(&layer)),
        (
            "https://example.com/SceneServer/layers/0/nodes/root",
            to_json(&root_node),
        ),
        (
            "https://example.com/SceneServer/layers/0/nodes/root/1",
            to_json(&child_node),
        ),
    ]);
    let loader = Arc::new(rest_loader(&registry));

    let root = block_on(loader.root()).expect("root");
    assert_eq!(root.index(), None, "a legacy node has no global index");
    assert_eq!(
        root.uri(),
        Some("https://example.com/SceneServer/layers/0/nodes/root")
    );
    let legacy = block_on(
        SceneLayerReader::new(Arc::clone(&loader)).legacy_node_uri(
            root.uri()
                .expect("legacy root retains its resolved non-numeric href"),
        ),
    )
    .expect("read typed legacy root by href");
    assert_eq!(&*legacy.id, "root");

    let expansion = block_on(loader.expand(root)).expect("expand root");
    let uris: Vec<&str> = expansion.contents.iter().map(|c| c.uri.as_str()).collect();
    assert_eq!(
        uris,
        vec![
            "https://example.com/SceneServer/layers/0/nodes/root/geometries/0",
            "https://example.com/SceneServer/layers/0/nodes/root/attributes/f_0/0",
            "https://example.com/SceneServer/layers/0/nodes/root/shared",
        ]
    );
    assert!(matches!(
        expansion.contents[0].kind,
        ContentKind::Legacy { role: "geometry" }
    ));

    // Hrefs chain: the child resolves its own hrefs relative to itself.
    let child = expansion.children[0].clone();
    assert_eq!(
        child.uri(),
        Some("https://example.com/SceneServer/layers/0/nodes/root/1")
    );
    let leaf = block_on(loader.expand(child.clone())).expect("expand child");
    assert_eq!(
        leaf.contents[0].uri,
        "https://example.com/SceneServer/layers/0/nodes/root/1/geometries/0"
    );

    // Node documents are retained, just like pages.
    block_on(loader.expand(child)).expect("re-expand child");
    assert_eq!(
        registry.request_count("https://example.com/SceneServer/layers/0/nodes/root/1"),
        1
    );
}

#[test]
fn falls_back_to_node_zero_when_a_legacy_layer_omits_its_root_node() {
    use i3s::cmn::{LayerType, NodeIndexDocument, SceneLayerInfo};

    // `store.rootNode` is optional. Without it there is no href to
    // resolve, so the conventional node 0 is the only thing left to try.
    let layer = SceneLayerInfo {
        id: 0,
        layer_type: LayerType::ThreeDObject,
        version: "1.6".into(),
        node_pages: None,
        ..Default::default()
    };
    let registry = Registry::with(vec![
        ("https://example.com/SceneServer/layers/0", to_json(&layer)),
        (
            "https://example.com/SceneServer/layers/0/nodes/0",
            to_json(&NodeIndexDocument {
                id: "root".into(),
                ..Default::default()
            }),
        ),
    ]);

    let root = block_on(rest_loader(&registry).root()).expect("root");
    assert_eq!(
        root.uri(),
        Some("https://example.com/SceneServer/layers/0/nodes/0")
    );
}

#[test]
fn reports_a_missing_layer_document_as_a_fetch_error() {
    let loader = rest_loader(&Registry::default());
    let error = block_on(loader.root()).expect_err("missing layer must fail");
    assert!(matches!(error, i3s::Error::Fetch { .. }), "got {error:?}");
}

#[test]
fn reports_a_zero_nodes_per_page_rather_than_dividing_by_it() {
    use i3s::cmn::{LayerType, NodePageDefinition, SceneLayerInfo};

    let layer = SceneLayerInfo {
        id: 4,
        layer_type: LayerType::ThreeDObject,
        version: "1.7".into(),
        node_pages: Some(NodePageDefinition {
            nodes_per_page: 0,
            ..Default::default()
        }),
        ..Default::default()
    };
    let registry = Registry::with(vec![(
        "https://example.com/SceneServer/layers/0",
        to_json(&layer),
    )]);
    let loader = rest_loader(&registry);

    match block_on(loader.root()).expect_err("zero page size must fail") {
        i3s::Error::InvalidNodesPerPage { layer_id, .. } => assert_eq!(layer_id, 4),
        other => panic!("expected InvalidNodesPerPage, got {other:?}"),
    }
}

#[test]
fn reports_a_node_missing_from_its_page() {
    use i3s::cmn::{LayerType, Node, NodePage, NodePageDefinition, Obb, SceneLayerInfo};

    // `rootIndex` 5 lands in page 2 at offset 1, but the page holds only
    // one node, so the tree and the pages disagree.
    let layer = SceneLayerInfo {
        id: 0,
        layer_type: LayerType::ThreeDObject,
        version: "1.7".into(),
        node_pages: Some(NodePageDefinition {
            nodes_per_page: 2,
            root_index: Some(5),
            ..Default::default()
        }),
        ..Default::default()
    };
    let page = NodePage {
        nodes: vec![Node {
            index: 4,
            obb: Obb {
                center: [0.0, 0.0, 0.0],
                half_size: [1.5, 1.5, 1.5],
                quaternion: [0.0, 0.0, 0.0, 1.0],
            },
            ..Default::default()
        }],
    };
    let registry = Registry::with(vec![
        ("https://example.com/SceneServer/layers/0", to_json(&layer)),
        (
            "https://example.com/SceneServer/layers/0/nodepages/2",
            to_json(&page),
        ),
    ]);
    let loader = rest_loader(&registry);

    let root = block_on(loader.root()).expect("root");
    match block_on(loader.expand(root)).expect_err("missing node must fail") {
        i3s::Error::MissingNode {
            node_index,
            page_id,
        } => {
            assert_eq!(node_index, 5);
            assert_eq!(page_id, 2);
        }
        other => panic!("expected MissingNode, got {other:?}"),
    }
}
