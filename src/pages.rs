//! Node-page indexing and packing.
//!
//! The specification stores nodes as a flat array served in fixed-size pages.
//! Producers that build a hierarchy node by node need the inverse of the
//! reader's page lookup: group nodes into pages without materializing them all.

use serde::Serialize;

use crate::Error;

/// Page index holding the node at `node_index`.
///
/// Implements `floor(node_index / nodes_per_page)`.
///
/// ```
/// # use i3s::page_id_of;
/// assert_eq!(page_id_of(0, 64), 0);
/// assert_eq!(page_id_of(63, 64), 0);
/// assert_eq!(page_id_of(64, 64), 1);
/// ```
///
/// # Panics
///
/// Panics if `nodes_per_page` is zero.
pub fn page_id_of(node_index: u64, nodes_per_page: u64) -> u64 {
    node_index / nodes_per_page
}

/// Offset of the node at `node_index` within its page.
///
/// ```
/// # use i3s::page_offset_of;
/// assert_eq!(page_offset_of(65, 64), 1);
/// ```
///
/// # Panics
///
/// Panics if `nodes_per_page` is zero.
pub fn page_offset_of(node_index: u64, nodes_per_page: u64) -> usize {
    (node_index % nodes_per_page) as usize
}

/// The serialized shape of one `nodepages/N.json` document.
#[derive(Serialize)]
struct Page<'a, T> {
    nodes: &'a [T],
}

/// How many pages `node_count` nodes occupy at `nodes_per_page` per page.
///
/// ```
/// # use i3s::node_page_count;
/// assert_eq!(node_page_count(0, 64), 0);
/// assert_eq!(node_page_count(64, 64), 1);
/// assert_eq!(node_page_count(65, 64), 2);
/// ```
///
/// # Panics
///
/// Panics if `nodes_per_page` is zero.
pub fn node_page_count(node_count: u64, nodes_per_page: u32) -> u64 {
    node_count.div_ceil(u64::from(nodes_per_page))
}

/// Groups `nodes` into serialized node-page documents.
///
/// Yields `(page_id, json)` for each page. Every page holds exactly
/// `nodes_per_page` nodes except the last, as the specification requires, and
/// `nodes` is consumed lazily, holding one page at a time.
///
/// `T` is any serializable node: the generated [`cmn::Node`](crate::cmn::Node),
/// or a `serde_json::Value` when a rewrite must preserve unknown members.
///
/// ```
/// # use i3s::pack_node_pages;
/// let pages: Vec<_> = pack_node_pages(0..5, 2)
///     .unwrap()
///     .collect::<Result<_, _>>()
///     .unwrap();
/// assert_eq!(pages[0], (0, br#"{"nodes":[0,1]}"#.to_vec()));
/// assert_eq!(pages[2], (2, br#"{"nodes":[4]}"#.to_vec()));
/// ```
///
/// # Errors
///
/// Returns [`Error::InvalidRewrite`] when `nodes_per_page` is not a power of
/// two below 4096, and an [`Error::Parse`] item if a node fails to serialize.
pub fn pack_node_pages<T, I>(
    nodes: I,
    nodes_per_page: u32,
) -> Result<impl Iterator<Item = Result<(u64, Vec<u8>), Error>>, Error>
where
    T: Serialize,
    I: IntoIterator<Item = T>,
{
    if !nodes_per_page.is_power_of_two() || nodes_per_page >= 4096 {
        return Err(Error::InvalidRewrite {
            message: format!(
                "nodesPerPage must be a power of two below 4096, got {nodes_per_page}"
            ),
        });
    }
    let mut nodes = nodes.into_iter();
    let mut next_page = 0u64;
    Ok(std::iter::from_fn(move || {
        let chunk: Vec<T> = nodes.by_ref().take(nodes_per_page as usize).collect();
        if chunk.is_empty() {
            return None;
        }
        let id = next_page;
        next_page += 1;
        Some(
            serde_json::to_vec(&Page { nodes: &chunk })
                .map(|body| (id, body))
                .map_err(|source| Error::json(format!("nodepages/{id}"), source)),
        )
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_math_partitions_node_indices() {
        for size in [1, 2, 64, 2048] {
            for index in 0..5000 {
                let page = page_id_of(index, size);
                let offset = page_offset_of(index, size) as u64;
                assert!(offset < size);
                assert_eq!(page * size + offset, index);
            }
        }
    }

    #[test]
    fn rejects_page_sizes_the_specification_forbids() {
        for bad in [0, 3, 4096, 8192] {
            assert!(pack_node_pages(0..1, bad).is_err(), "{bad}");
        }
        assert!(pack_node_pages(0..1, 2048).is_ok());
    }

    #[test]
    fn no_nodes_means_no_pages() {
        assert_eq!(pack_node_pages(0..0, 64).unwrap().count(), 0);
    }

    #[test]
    fn page_ids_agree_with_the_reader_lookup() {
        let pages: Vec<_> = pack_node_pages(0..300u32, 64)
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(pages.len() as u64, node_page_count(300, 64));
        for (id, _) in &pages {
            assert_eq!(crate::page_id_of(id * 64, 64), *id);
        }
    }
}
