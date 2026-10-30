# I3S

Rust implementation of Esri's
[Indexed 3D Scene Layers (I3S)](https://github.com/Esri/i3s-spec) format.

## Installation

```toml
[dependencies]
i3s = "0.2"
```

Requires Rust 1.85.

## What's included

- Typed documents for common definitions (`cmn`), buildings (`bld`),
  3D objects and points (`psl`), and point clouds (`pcsl`).
- Node and sublayer identities, SLPK hash canonicalization, and node-page arithmetic.
- Decoding and encoding of uncompressed geometry and attribute buffers.

## Node pages and package hashes

```rust
use i3s::{slpk_hash_path, page_id_of, page_offset_of};

assert_eq!(slpk_hash_path("/Nodes/6/Geometries/0.bin"), "nodes/6/geometries/0.bin");
assert_eq!(page_id_of(65, 64), 1);
assert_eq!(page_offset_of(65, 64), 1);
```

`slpk_hash_path` applies the SLPK specification's hash-table naming rule;
it does not resolve addresses or access storage.

## Documents and binary data

Use `SceneLayerDocument` and `NodePageDocument` for parsed control documents.
The schema types support Serde serialization and deserialization.

`decode_geometry` and `encode_geometry_checked` use a geometry definition and
explicit counts. `decode_schema_geometry` handles the legacy geometry layout.
`decode_attribute` and `encode_attribute` handle attribute storage layouts.
Compressed geometry requires a separate codec.

See the [API documentation](https://docs.rs/i3s) for signatures and examples.

## Development

Generated types are checked in. `generate.ps1` regenerates them using the
development-only `xtask` toolchain and schema submodule.

## License

Apache License 2.0. See [LICENSE](LICENSE).
