//! URL construction helpers for I3S REST endpoints.
//!
//! All functions take a `base_url` that is the root of the scene service
//! (i.e., the URL that returns the `SceneLayerInfo` JSON when you append
//! `/layers/{layer_id}`). Trailing slashes are normalised.
//!
//! # Resource ids are not node indices
//!
//! Every node-scoped resource URL is keyed by a **resource id**, taken from
//! the node's `mesh` sub-objects (`mesh.geometry.resource`,
//! `mesh.material.resource`, `mesh.attribute.resource`) - *not* by
//! [`Node::index`](crate::cmn::Node::index). The specification is explicit
//! that these need not agree:
//!
//! > Clients have to use the `resource` identifiers written in each node to
//! > access the resources. While content creator may choose to match
//! > `resource` with the node id this is not required by the I3S
//! > specification and clients should not make this assumption.
//! >
//! > -- `mesh.cmn.md`

/// URL for the `SceneLayerInfo` of `layer_id`.
///
/// ```
/// # use i3s::layer_url;
/// assert_eq!(layer_url("https://example.com/sls", 0), "https://example.com/sls/layers/0");
/// ```
pub fn layer_url(base_url: &str, layer_id: i64) -> String {
    format!("{}/layers/{layer_id}", base_url.trim_end_matches('/'))
}

/// URL for the node page at `page_id`.
///
/// A page holds `layer.nodePages.nodesPerPage` nodes; derive `page_id` from a
/// node index with [`page_id_of`](crate::page_id_of).
///
/// ```
/// # use i3s::node_page_url;
/// assert_eq!(
///     node_page_url("https://example.com/sls", 0, 3),
///     "https://example.com/sls/layers/0/nodepages/3"
/// );
/// ```
pub fn node_page_url(base_url: &str, layer_id: i64, page_id: u64) -> String {
    format!(
        "{}/layers/{layer_id}/nodepages/{page_id}",
        base_url.trim_end_matches('/')
    )
}

/// URL for a geometry resource.
///
/// `resource` is `mesh.geometry.resource` for the node - see the
/// [module docs](self) on why this is not the node index. `buffer_index` is
/// the index of the chosen `GeometryBuffer` within the `GeometryDefinition`
/// named by `mesh.geometry.definition`.
///
/// ```
/// # use i3s::geometry_url;
/// assert_eq!(
///     geometry_url("https://example.com/sls", 0, 6, 1),
///     "https://example.com/sls/layers/0/nodes/6/geometries/1"
/// );
/// ```
pub fn geometry_url(base_url: &str, layer_id: i64, resource: i64, buffer_index: usize) -> String {
    format!(
        "{}/layers/{layer_id}/nodes/{resource}/geometries/{buffer_index}",
        base_url.trim_end_matches('/')
    )
}

/// URL for an attribute resource.
///
/// `resource` is `mesh.attribute.resource` for the node. `attribute_index` is
/// the zero-based index of the attribute in
/// [`SceneLayerInfo::attribute_storage_info`](crate::cmn::SceneLayerInfo::attribute_storage_info).
///
/// ```
/// # use i3s::attribute_url;
/// assert_eq!(
///     attribute_url("https://example.com/sls", 0, 6, 0),
///     "https://example.com/sls/layers/0/nodes/6/attributes/f_0/0"
/// );
/// ```
pub fn attribute_url(
    base_url: &str,
    layer_id: i64,
    resource: i64,
    attribute_index: usize,
) -> String {
    format!(
        "{}/layers/{layer_id}/nodes/{resource}/attributes/f_{attribute_index}/0",
        base_url.trim_end_matches('/')
    )
}

/// URL for a texture resource.
///
/// `resource` is `mesh.material.resource` for the node. `name` is the format
/// name declared by the layer's `textureSetDefinitions[..].formats[..].name`
/// (for example `"0"`, `"0_0_1"`, `"0_0_2"`, `"1"`).
///
/// ```
/// # use i3s::texture_url;
/// assert_eq!(
///     texture_url("https://example.com/sls", 0, 6, "0_0_1"),
///     "https://example.com/sls/layers/0/nodes/6/textures/0_0_1"
/// );
/// ```
pub fn texture_url(base_url: &str, layer_id: i64, resource: i64, name: &str) -> String {
    format!(
        "{}/layers/{layer_id}/nodes/{resource}/textures/{name}",
        base_url.trim_end_matches('/')
    )
}

/// Page index holding the node at `node_index`.
///
/// > `page_id = floor( node_id / node_per_page )`
/// >
/// > -- `nodePageDefinition.cmn.md`
///
/// ```
/// # use i3s::page_id_of;
/// assert_eq!(page_id_of(0, 64), 0);
/// assert_eq!(page_id_of(63, 64), 0);
/// assert_eq!(page_id_of(64, 64), 1);
/// ```
pub fn page_id_of(node_index: u64, nodes_per_page: u64) -> u64 {
    node_index / nodes_per_page
}

/// Offset of the node at `node_index` within its page.
///
/// > `node_id_in_page = modulo( node_id, node_per_page )`
/// >
/// > -- `nodePageDefinition.cmn.md`
///
/// ```
/// # use i3s::page_offset_of;
/// assert_eq!(page_offset_of(65, 64), 1);
/// ```
pub fn page_offset_of(node_index: u64, nodes_per_page: u64) -> usize {
    (node_index % nodes_per_page) as usize
}
