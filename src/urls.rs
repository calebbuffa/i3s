//! URL and path construction for the three ways an I3S layer is published.
//!
//! I3S defines one logical resource tree, but three transports expose it
//! differently, and the difference is entirely in how a resource is named:
//!
//! | Source | Naming |
//! | --- | --- |
//! | [`SourceKind::Rest`] | `http://serviceURL/layers/{layerID}/nodepages/{id}` |
//! | [`SourceKind::Slpk`] | `nodepages/{id}.json.gz` — an entry inside a ZIP |
//! | [`SourceKind::Eslpk`] | `{base}/nodepages/{id}.json.gz` — the same tree unpacked |
//!
//! The REST templates come from the profile ReadMe files (for example
//! `3Dobject_ReadMe.md`); the package layout comes from the *scene layer
//! structure* diagram in the same documents.
//!
//! Two consequences follow from the package layout and are handled here:
//!
//! * **Packaged resources carry a file extension** that REST omits — `.json`
//!   for documents, `.bin` for binary, and a `.gz` suffix when the store
//!   declares gzip in `nidEncoding`/`featureEncoding`.
//! * **The layer id disappears** inside a package: a package holds exactly one
//!   layer, so its tree is rooted at the layer rather than at `/layers/{id}`.
//!
//! # Resource ids are not node indices
//!
//! Every node-scoped resource is keyed by a **resource id**, taken from the
//! node's `mesh` sub-objects (`mesh.geometry.resource`,
//! `mesh.material.resource`, `mesh.attribute.resource`) — *not* by
//! [`Node::index`](crate::cmn::Node::index):
//!
//! > Clients have to use the `resource` identifiers written in each node to
//! > access the resources. While content creator may choose to match
//! > `resource` with the node id this is not required by the I3S
//! > specification and clients should not make this assumption.
//! >
//! > -- `mesh.cmn.md`

/// How a scene layer is published, which determines how resources are named.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SourceKind {
    /// A live I3S REST service (`.../SceneServer/layers/{layerID}/...`).
    #[default]
    Rest,
    /// A scene layer package: entries inside a `.slpk` ZIP archive.
    ///
    /// Produces archive-relative paths. Per `slpk_hashtable.cmn.md` these are
    /// converted to a canonical form with [`canonical_path`] before hashing.
    Slpk,
    /// An *exploded* scene layer package: the SLPK tree unpacked onto disk or
    /// served statically, so package-style names resolve against a base.
    Eslpk,
}

impl SourceKind {
    /// Whether resources are named the packaged way (extensions, no
    /// `/layers/{id}` prefix).
    fn packaged(self) -> bool {
        matches!(self, Self::Slpk | Self::Eslpk)
    }
}

/// Names resources for one scene layer from one source.
///
/// ```
/// # use i3s::Source;
/// let rest = Source::rest("https://example.com/SceneServer", 0);
/// assert_eq!(rest.node_page(3), "https://example.com/SceneServer/layers/0/nodepages/3");
///
/// let slpk = Source::slpk();
/// assert_eq!(slpk.node_page(3), "nodepages/3.json.gz");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    base_url: String,
    layer_id: u32,
    /// Set when this source addresses a building-scene-layer sublayer, whose
    /// resources hang below `/sublayers/{id}` rather than off the layer root.
    sublayer_id: Option<u32>,
    kind: SourceKind,
    gzip: bool,
}

impl Default for Source {
    fn default() -> Self {
        Self::rest(String::new(), 0)
    }
}

impl Source {
    /// A live REST service rooted at `base_url`, serving `layer_id`.
    pub fn rest(base_url: impl Into<String>, layer_id: u32) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            layer_id,
            sublayer_id: None,
            kind: SourceKind::Rest,
            gzip: false,
        }
    }

    /// A `.slpk` archive, whose entries are named relative to the archive
    /// root. Gzip is enabled by default because packages store JSON gzipped.
    pub fn slpk() -> Self {
        Self {
            base_url: String::new(),
            layer_id: 0,
            sublayer_id: None,
            kind: SourceKind::Slpk,
            gzip: true,
        }
    }

    /// An exploded package rooted at `base_url`.
    pub fn eslpk(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            layer_id: 0,
            sublayer_id: None,
            kind: SourceKind::Eslpk,
            gzip: true,
        }
    }

    /// Sets whether packaged documents carry a `.gz` suffix.
    ///
    /// Derive this from the store's `nidEncoding`/`featureEncoding`, which
    /// name a MIME type such as
    /// `application/vnd.esri.i3s.json+gzip; version=1.6`. Has no effect on a
    /// REST source, where gzip is negotiated by `Content-Encoding` instead.
    pub fn with_gzip(mut self, gzip: bool) -> Self {
        self.gzip = gzip;
        self
    }

    /// Sets the layer id. Only meaningful for [`SourceKind::Rest`]; a package
    /// contains exactly one layer and so has no layer segment.
    ///
    /// This is ignored once a sublayer has been selected: a sublayer's id is
    /// in a different namespace from its building layer's, and the building
    /// layer's id is the one that appears in the path.
    pub fn with_layer_id(mut self, layer_id: u32) -> Self {
        if self.sublayer_id.is_none() {
            self.layer_id = layer_id;
        }
        self
    }

    /// How this layer is published.
    pub fn kind(&self) -> SourceKind {
        self.kind
    }

    /// The layer id this source serves.
    pub fn layer_id(&self) -> u32 {
        self.layer_id
    }

    /// The sublayer id this source serves, for a building scene layer.
    pub fn sublayer_id(&self) -> Option<u32> {
        self.sublayer_id
    }

    /// Descends into a building-scene-layer sublayer.
    ///
    /// > **If** `layerType != 'group'`, resources will be at
    /// > `/layers/{bim_layer_id}/sublayers/{this.id}/...`
    /// >
    /// > -- `sublayer.bld.md`
    ///
    /// The returned source names that sublayer's own nodes, geometries and
    /// attributes, so the rest of the crate needs no notion of sublayers: a
    /// building layer is traversed by handing each sublayer its own `Source`.
    ///
    /// ```
    /// # use i3s::Source;
    /// let sublayer = Source::rest("https://example.com/sls", 0).sublayer(33);
    /// assert_eq!(
    ///     sublayer.geometry(0, 0),
    ///     "https://example.com/sls/layers/0/sublayers/33/nodes/0/geometries/0"
    /// );
    /// // Packages nest the same way, below the archive root.
    /// assert_eq!(
    ///     Source::slpk().sublayer(33).node_page(0),
    ///     "sublayers/33/nodepages/0.json.gz"
    /// );
    /// ```
    pub fn sublayer(&self, sublayer_id: u32) -> Self {
        let mut source = self.clone();
        if self.packaged() {
            // Inside a package a sublayer is just a subtree, so fold the
            // segment into the base and clear the marker: everything below
            // is named exactly like a standalone package layer.
            source.base_url = if self.base_url.is_empty() {
                format!("sublayers/{sublayer_id}")
            } else {
                format!("{}/sublayers/{sublayer_id}", self.base_url)
            };
            source.sublayer_id = Some(sublayer_id);
        } else {
            source.sublayer_id = Some(sublayer_id);
        }
        source
    }

    fn packaged(&self) -> bool {
        self.kind.packaged()
    }

    /// Joins `path` onto the source's root.
    ///
    /// An SLPK entry is named relative to the archive root, so an empty base
    /// yields a bare relative path; a sublayer sets a base even inside a
    /// package, which is why this is not special-cased per kind.
    fn join(&self, path: String) -> String {
        if self.base_url.is_empty() {
            path
        } else {
            format!("{}/{path}", self.base_url)
        }
    }

    /// Builds a packaged name, appending `.gz` when the store gzips documents.
    fn packaged_name(&self, stem: String, extension: &str) -> String {
        if self.gzip {
            self.join(format!("{stem}.{extension}.gz"))
        } else {
            self.join(format!("{stem}.{extension}"))
        }
    }

    /// Builds a REST path below the layer, or below the sublayer when this
    /// source addresses one.
    fn rest_path(&self, tail: std::fmt::Arguments<'_>) -> String {
        match self.sublayer_id {
            Some(sublayer) => format!(
                "{}/layers/{}/sublayers/{sublayer}/{tail}",
                self.base_url, self.layer_id
            ),
            None => format!("{}/layers/{}/{tail}", self.base_url, self.layer_id),
        }
    }

    /// The scene layer document.
    ///
    /// For a sublayer source this is the sublayer's own `3dSceneLayer.json`,
    /// which is a full layer document in its own right.
    ///
    /// ```
    /// # use i3s::Source;
    /// assert_eq!(
    ///     Source::rest("https://example.com/sls", 0).layer(),
    ///     "https://example.com/sls/layers/0"
    /// );
    /// assert_eq!(
    ///     Source::rest("https://example.com/sls", 0).sublayer(33).layer(),
    ///     "https://example.com/sls/layers/0/sublayers/33"
    /// );
    /// assert_eq!(Source::slpk().layer(), "3dSceneLayer.json.gz");
    /// assert_eq!(
    ///     Source::eslpk("file:///tmp/layer").with_gzip(false).layer(),
    ///     "file:///tmp/layer/3dSceneLayer.json"
    /// );
    /// ```
    pub fn layer(&self) -> String {
        if self.packaged() {
            self.packaged_name("3dSceneLayer".to_string(), "json")
        } else {
            match self.sublayer_id {
                Some(sublayer) => format!(
                    "{}/layers/{}/sublayers/{sublayer}",
                    self.base_url, self.layer_id
                ),
                None => format!("{}/layers/{}", self.base_url, self.layer_id),
            }
        }
    }

    /// The node page at `page_id`.
    ///
    /// Derive `page_id` from a node index with [`page_id_of`].
    pub fn node_page(&self, page_id: u64) -> String {
        if self.packaged() {
            self.packaged_name(format!("nodepages/{page_id}"), "json")
        } else {
            self.rest_path(format_args!("nodepages/{page_id}"))
        }
    }

    /// A geometry resource.
    ///
    /// `resource` is `mesh.geometry.resource` — see the [module docs](self) on
    /// why this is not the node index. `buffer_index` selects a
    /// `GeometryBuffer` within the node's `GeometryDefinition`.
    ///
    /// ```
    /// # use i3s::Source;
    /// assert_eq!(
    ///     Source::rest("https://example.com/sls", 0).geometry(6, 1),
    ///     "https://example.com/sls/layers/0/nodes/6/geometries/1"
    /// );
    /// assert_eq!(Source::slpk().geometry(6, 1), "nodes/6/geometries/1.bin.gz");
    /// ```
    pub fn geometry(&self, resource: u32, buffer_index: usize) -> String {
        if self.packaged() {
            self.packaged_name(format!("nodes/{resource}/geometries/{buffer_index}"), "bin")
        } else {
            self.rest_path(format_args!("nodes/{resource}/geometries/{buffer_index}"))
        }
    }

    /// An attribute resource.
    ///
    /// `attribute_key` is the *key* from
    /// [`AttributeStorageInfo::key`](crate::cmn::AttributeStorageInfo::key)
    /// (the `N` in `f_N`), not the array position — the spec allows these to
    /// differ, and gaps are common where fields have been dropped.
    ///
    /// ```
    /// # use i3s::Source;
    /// assert_eq!(
    ///     Source::rest("https://example.com/sls", 0).attribute(6, 8),
    ///     "https://example.com/sls/layers/0/nodes/6/attributes/f_8/0"
    /// );
    /// assert_eq!(Source::slpk().attribute(6, 8), "nodes/6/attributes/f_8/0.bin.gz");
    /// ```
    pub fn attribute(&self, resource: u32, attribute_key: usize) -> String {
        if self.packaged() {
            self.packaged_name(
                format!("nodes/{resource}/attributes/f_{attribute_key}/0"),
                "bin",
            )
        } else {
            self.rest_path(format_args!(
                "nodes/{resource}/attributes/f_{attribute_key}/0"
            ))
        }
    }

    /// A texture resource.
    ///
    /// `name` is the format name from
    /// `textureSetDefinitions[..].formats[..].name` (for example `"0"`,
    /// `"0_0_1"`, `"1"`). Packages append the format's own extension rather
    /// than a generic one, because a texture's encoding is part of its name:
    ///
    /// | File name | Format |
    /// | --- | --- |
    /// | `0_0.jpg` | JPEG |
    /// | `0.bin` | PNG |
    /// | `0_0_1.bin.dds` | S3TC |
    /// | `0_0_2.ktx` | ETC2 |
    /// | `1.ktx2` | Basis Universal |
    ///
    /// > -- `texture.cmn.md`
    ///
    /// Textures are already compressed, so no `.gz` suffix is added.
    ///
    /// ```
    /// # use i3s::Source;
    /// assert_eq!(
    ///     Source::rest("https://example.com/sls", 0).texture(6, "0_0_1", "dds"),
    ///     "https://example.com/sls/layers/0/nodes/6/textures/0_0_1"
    /// );
    /// assert_eq!(
    ///     Source::slpk().texture(6, "0_0_1", "dds"),
    ///     "nodes/6/textures/0_0_1.bin.dds"
    /// );
    /// assert_eq!(Source::slpk().texture(6, "0_0", "jpg"), "nodes/6/textures/0_0.jpg");
    /// ```
    pub fn texture(&self, resource: u32, name: &str, format_extension: &str) -> String {
        if self.packaged() {
            // S3TC keeps the `.bin` stem that its `image/vnd-ms.dds` MIME
            // type implies; the other formats replace it outright.
            let file = match format_extension {
                "dds" => format!("{name}.bin.dds"),
                "bin" | "" => format!("{name}.bin"),
                extension => format!("{name}.{extension}"),
            };
            self.join(format!("nodes/{resource}/textures/{file}"))
        } else {
            self.rest_path(format_args!("nodes/{resource}/textures/{name}"))
        }
    }

    /// The statistics resource for an attribute key.
    ///
    /// ```
    /// # use i3s::Source;
    /// assert_eq!(
    ///     Source::rest("https://example.com/sls", 0).statistics(8),
    ///     "https://example.com/sls/layers/0/statistics/f_8/0"
    /// );
    /// ```
    pub fn statistics(&self, attribute_key: usize) -> String {
        if self.packaged() {
            self.packaged_name(format!("statistics/f_{attribute_key}/0"), "json")
        } else {
            self.rest_path(format_args!("statistics/f_{attribute_key}/0"))
        }
    }

    /// The building-scene-layer summary statistics resource.
    ///
    /// > `http://serviceURL/layers/{layerID}/statistics/summary`
    /// >
    /// > -- `BSL_ReadMe.md`
    ///
    /// Distinct from [`statistics`](Self::statistics), which is per-attribute.
    ///
    /// ```
    /// # use i3s::Source;
    /// assert_eq!(
    ///     Source::rest("https://example.com/sls", 0).statistics_summary(),
    ///     "https://example.com/sls/layers/0/statistics/summary"
    /// );
    /// ```
    pub fn statistics_summary(&self) -> String {
        if self.packaged() {
            self.packaged_name("statistics/summary".to_string(), "json")
        } else {
            self.rest_path(format_args!("statistics/summary"))
        }
    }

    /// A `featureData` resource.
    ///
    /// > `http://serviceURL/layers/{layerID}/nodes/{nodeID}/features/0`
    ///
    /// Only required for 1.6 compatibility, but still published by current
    /// writers, and the only place per-feature geometry lives in a 1.6 store.
    ///
    /// ```
    /// # use i3s::Source;
    /// assert_eq!(
    ///     Source::rest("https://example.com/sls", 0).feature_data(6, 0),
    ///     "https://example.com/sls/layers/0/nodes/6/features/0"
    /// );
    /// assert_eq!(Source::slpk().feature_data(6, 0), "nodes/6/features/0.json.gz");
    /// ```
    pub fn feature_data(&self, resource: u32, feature_index: usize) -> String {
        if self.packaged() {
            self.packaged_name(format!("nodes/{resource}/features/{feature_index}"), "json")
        } else {
            self.rest_path(format_args!("nodes/{resource}/features/{feature_index}"))
        }
    }

    /// A 1.6 `3dNodeIndexDocument`, used when the layer has no `nodePages`.
    ///
    /// Here the id genuinely *is* the node index: 1.6 addressed nodes
    /// directly, and the resource/node split only arrived with 1.7.
    ///
    /// ```
    /// # use i3s::Source;
    /// assert_eq!(
    ///     Source::rest("https://example.com/sls", 0).node(6),
    ///     "https://example.com/sls/layers/0/nodes/6"
    /// );
    /// assert_eq!(Source::slpk().node(6), "nodes/6/3dNodeIndexDocument.json.gz");
    /// ```
    pub fn node(&self, node_id: u32) -> String {
        if self.packaged() {
            self.packaged_name(format!("nodes/{node_id}/3dNodeIndexDocument"), "json")
        } else {
            self.rest_path(format_args!("nodes/{node_id}"))
        }
    }

    /// A 1.6 `sharedResource` document.
    ///
    /// ```
    /// # use i3s::Source;
    /// assert_eq!(
    ///     Source::rest("https://example.com/sls", 0).shared(6),
    ///     "https://example.com/sls/layers/0/nodes/6/shared"
    /// );
    /// ```
    pub fn shared(&self, node_id: u32) -> String {
        if self.packaged() {
            self.packaged_name(format!("nodes/{node_id}/shared/sharedResource"), "json")
        } else {
            self.rest_path(format_args!("nodes/{node_id}/shared"))
        }
    }
}

/// Converts a package path to the canonical form used by the SLPK hash table.
///
/// > Canonical paths must use a forward slash as the path separator `/`, not
/// > contain a heading forward slash. Example: `/my/PATH.json` converts to
/// > `my/path.json`.
/// >
/// > -- `slpk_hashtable.cmn.md`
///
/// ```
/// # use i3s::canonical_path;
/// assert_eq!(canonical_path("/my/PATH.json"), "my/path.json");
/// assert_eq!(canonical_path(r"nodes\0\Geometries\0.bin"), "nodes/0/geometries/0.bin");
/// ```
pub fn canonical_path(path: &str) -> String {
    path.replace('\\', "/")
        .trim_start_matches('/')
        .to_lowercase()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rest_keeps_the_layer_segment_and_omits_extensions() {
        let source = Source::rest("https://example.com/sls/", 2);
        assert_eq!(source.layer(), "https://example.com/sls/layers/2");
        assert_eq!(
            source.node_page(7),
            "https://example.com/sls/layers/2/nodepages/7"
        );
        assert_eq!(
            source.geometry(6, 0),
            "https://example.com/sls/layers/2/nodes/6/geometries/0"
        );
    }

    #[test]
    fn packages_drop_the_layer_segment() {
        // A package contains exactly one layer, so `/layers/{id}` has no
        // meaning inside it even when a layer id was supplied.
        let source = Source::slpk().with_layer_id(3);
        assert_eq!(source.layer(), "3dSceneLayer.json.gz");
        assert_eq!(source.node_page(7), "nodepages/7.json.gz");
    }

    #[test]
    fn gzip_suffix_is_configurable() {
        let source = Source::slpk().with_gzip(false);
        assert_eq!(source.layer(), "3dSceneLayer.json");
        assert_eq!(source.geometry(6, 0), "nodes/6/geometries/0.bin");
    }

    #[test]
    fn gzip_does_not_affect_rest() {
        // REST negotiates compression via `Content-Encoding`, so a `.gz`
        // suffix would be a 404.
        let source = Source::rest("https://example.com/sls", 0).with_gzip(true);
        assert_eq!(source.layer(), "https://example.com/sls/layers/0");
    }

    #[test]
    fn eslpk_resolves_package_names_against_a_base() {
        let source = Source::eslpk("file:///tmp/layer/");
        assert_eq!(source.layer(), "file:///tmp/layer/3dSceneLayer.json.gz");
        assert_eq!(
            source.geometry(6, 1),
            "file:///tmp/layer/nodes/6/geometries/1.bin.gz"
        );
    }

    #[test]
    fn textures_use_their_format_extension_not_gzip() {
        let source = Source::slpk();
        assert_eq!(source.texture(6, "0_0", "jpg"), "nodes/6/textures/0_0.jpg");
        assert_eq!(source.texture(6, "0", "bin"), "nodes/6/textures/0.bin");
        assert_eq!(
            source.texture(6, "0_0_1", "dds"),
            "nodes/6/textures/0_0_1.bin.dds"
        );
        assert_eq!(source.texture(6, "1", "ktx2"), "nodes/6/textures/1.ktx2");
    }

    #[test]
    fn canonical_path_matches_the_hash_table_rule() {
        assert_eq!(canonical_path("/my/PATH.json"), "my/path.json");
        assert_eq!(canonical_path("my/path.json"), "my/path.json");
    }

    #[test]
    fn page_math_partitions_node_indices() {
        for index in 0u64..200 {
            let page = page_id_of(index, 64);
            let offset = page_offset_of(index, 64) as u64;
            assert_eq!(page * 64 + offset, index);
        }
    }

    #[test]
    fn rest_sublayers_nest_below_the_building_layer() {
        // A building scene layer's own id and its sublayer's id are in
        // different namespaces, and both appear in the path.
        let source = Source::rest("https://example.com/sls", 10).sublayer(33);
        assert_eq!(source.layer_id(), 10);
        assert_eq!(source.sublayer_id(), Some(33));
        assert_eq!(
            source.layer(),
            "https://example.com/sls/layers/10/sublayers/33"
        );
        assert_eq!(
            source.node_page(0),
            "https://example.com/sls/layers/10/sublayers/33/nodepages/0"
        );
        assert_eq!(
            source.attribute(4, 2),
            "https://example.com/sls/layers/10/sublayers/33/nodes/4/attributes/f_2/0"
        );
    }

    #[test]
    fn packaged_sublayers_become_a_subtree() {
        let source = Source::slpk().sublayer(33);
        assert_eq!(source.layer(), "sublayers/33/3dSceneLayer.json.gz");
        assert_eq!(
            source.geometry(0, 0),
            "sublayers/33/nodes/0/geometries/0.bin.gz"
        );
        assert_eq!(
            Source::eslpk("file:///tmp/bsl").sublayer(1).node_page(2),
            "file:///tmp/bsl/sublayers/1/nodepages/2.json.gz"
        );
    }

    #[test]
    fn summary_statistics_differ_from_per_attribute_statistics() {
        let source = Source::rest("https://example.com/sls", 0);
        assert_eq!(
            source.statistics_summary(),
            "https://example.com/sls/layers/0/statistics/summary"
        );
        assert_eq!(
            source.statistics(2),
            "https://example.com/sls/layers/0/statistics/f_2/0"
        );
    }
}
