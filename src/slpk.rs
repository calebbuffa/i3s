//! SLPK hash-table path canonicalization.

/// Converts a package path to the canonical form used by the SLPK hash table.
///
/// > Canonical paths must use a forward slash as the path separator `/`, not
/// > contain a heading forward slash. Example: `/my/PATH.json` converts to
/// > `my/path.json`.
/// >
/// > -- `slpk_hashtable.cmn.md`
///
/// ```
/// # use i3s::slpk_hash_path;
/// assert_eq!(slpk_hash_path("/my/PATH.json"), "my/path.json");
/// assert_eq!(slpk_hash_path(r"nodes\0\Geometries\0.bin"), "nodes/0/geometries/0.bin");
/// ```
pub fn slpk_hash_path(path: &str) -> String {
    path.replace('\\', "/")
        .trim_start_matches('/')
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_path_matches_the_specification() {
        assert_eq!(slpk_hash_path("/my/PATH.json"), "my/path.json");
        assert_eq!(
            slpk_hash_path(r"nodes\0\Geometries\0.bin"),
            "nodes/0/geometries/0.bin"
        );
    }

    #[test]
    fn hash_path_is_idempotent() {
        for path in ["/my/PATH.json", r"nodes\0\Geometries\0.bin", "nodes/0", ""] {
            let canonical = slpk_hash_path(path);
            assert_eq!(slpk_hash_path(&canonical), canonical);
        }
    }
}
