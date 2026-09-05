//! Parses layer documents captured verbatim from published I3S services.
//!
//! The hand-built fixtures in `loader.rs` prove the loader's logic, but they
//! are written against the *specification*, so they cannot catch the places
//! where the specification and shipped data disagree — and there are such
//! places. These documents were fetched from live ArcGIS services and are
//! checked in unmodified, so a regression against real data fails here.
//!
//! Each file records where it came from:
//!
//! * `point_osm_trees.json` — OpenStreetMap 3D Trees (Thematic),
//!   `basemaps3d.arcgis.com/.../OpenStreetMap3D_Trees_Thematic_v1`. A modern
//!   node-paged Point layer, store version 2.0.
//! * `building_turanga.json` — Turanga Library,
//!   `tiles.arcgis.com/tiles/cFEFS0EWrhfDeVw9/.../Turanga_Library`. A
//!   building scene layer with nested groups and mixed leaf types.
//! * `object_sublayer.json` / `point_legacy_sublayer.json` — two sublayers
//!   of that building layer, which are 1.6 layers in their own right.

use i3s::{bld, cmn, psl};

fn fixture(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/realworld")
        .join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn parses_a_published_point_layer() {
    let layer: psl::SceneLayerInfoPsl =
        serde_json::from_str(&fixture("point_osm_trees.json")).expect("parse point layer");

    assert_eq!(layer.layer_type, psl::LayerType::Point);
    assert_eq!(&*layer.store.version, "2.0");

    // The page index is named `pointNodePages`, and real services omit
    // `rootIndex` -- so a loader that requires it would reject this layer.
    let pages = layer.point_node_pages.as_ref().expect("pointNodePages");
    assert_eq!(pages.nodes_per_page, 84);
    assert_eq!(pages.root_index, None);

    // The spec's psl property table calls this `geometryDefinition` and
    // types it as a single object. Published data disagrees, which is why
    // `i3s-schema` carries an erratum for it.
    assert_eq!(layer.geometry_definitions.len(), 1);
    let buffers = &layer.geometry_definitions[0].geometry_buffers;
    assert_eq!(buffers.len(), 1);
    let compressed = buffers[0]
        .compressed_attributes
        .as_ref()
        .expect("point geometry is always compressed");
    assert_eq!(
        compressed.encoding,
        cmn::CompressedAttributesEncoding::Draco
    );

    // Legacy roots are named, not indexed.
    assert_eq!(layer.store.root_node.as_deref(), Some("./nodes/root"));
    assert!(!layer.attribute_storage_info.is_empty());
}

#[test]
fn parses_a_published_building_layer() {
    let layer: bld::LayerBld =
        serde_json::from_str(&fixture("building_turanga.json")).expect("parse building layer");

    assert_eq!(layer.layer_type, bld::LayerBldLayerType::Building);
    assert_eq!(&*layer.name, "NCL-AR-Building-RVT15");

    // Two top-level entries: a `group` holding the full model, and a
    // resource-bearing `3DObject` overview sublayer.
    assert_eq!(layer.sublayers.len(), 2);

    let overview = layer
        .sublayers
        .iter()
        .find(|s| s.layer_type == bld::SublayerBldLayerType::ThreeDObject)
        .expect("an overview sublayer");
    // A sublayer id shares no namespace with the layer id: both are 0 here,
    // which is exactly the collision that must not corrupt a URL.
    assert_eq!(overview.id, 0);
    assert_eq!(layer.id, 0);

    // Groups nest, and the leaves are of mixed type.
    let (groups, leaves) = count_sublayers(&layer.sublayers);
    assert_eq!(
        groups, 4,
        "Full Model + Electrical/Structural/Architectural"
    );
    assert!(leaves > 15);
    assert!(
        collect_types(&layer.sublayers).contains(&bld::SublayerBldLayerType::Point),
        "this model mixes a Point sublayer in among the 3DObject ones"
    );
}

#[test]
fn published_building_sublayers_are_layers_in_their_own_right() {
    // A `3DObject` sublayer parses as an ordinary mesh layer...
    let object: cmn::SceneLayerInfo =
        serde_json::from_str(&fixture("object_sublayer.json")).expect("parse object sublayer");
    assert_eq!(object.id, 19);
    assert_eq!(object.layer_type, cmn::LayerType::ThreeDObject);

    // ...and a `Point` sublayer as a point layer. Both are 1.6: they have no
    // node pages at all, so the loader must fall back to legacy traversal
    // rooted at `store.rootNode` rather than assuming node 0.
    let point: psl::SceneLayerInfoPsl =
        serde_json::from_str(&fixture("point_legacy_sublayer.json")).expect("parse point sublayer");
    assert_eq!(point.id, 11);
    assert_eq!(point.layer_type, psl::LayerType::Point);
    assert!(point.point_node_pages.is_none());
    assert_eq!(&*point.store.version, "1.6");
    assert_eq!(point.store.root_node.as_deref(), Some("./nodes/root"));
}

fn count_sublayers(sublayers: &[bld::SublayerBld]) -> (usize, usize) {
    let (mut groups, mut leaves) = (0, 0);
    for sublayer in sublayers {
        if sublayer.layer_type == bld::SublayerBldLayerType::Group {
            groups += 1;
            let (g, l) = count_sublayers(&sublayer.sublayers);
            groups += g;
            leaves += l;
        } else {
            leaves += 1;
        }
    }
    (groups, leaves)
}

fn collect_types(sublayers: &[bld::SublayerBld]) -> Vec<bld::SublayerBldLayerType> {
    let mut types = Vec::new();
    for sublayer in sublayers {
        types.push(sublayer.layer_type);
        types.extend(collect_types(&sublayer.sublayers));
    }
    types
}
