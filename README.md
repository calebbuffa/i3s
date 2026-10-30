# I3S

The [Indexed 3D Scene Layers specification](https://github.com/Esri/i3s-spec)
as Rust data and common wire-format utilities.

The crate provides generated document types (`cmn`, `bld`, `psl`, `pcsl`),
resource identities and naming, node-page arithmetic, and uncompressed geometry
and attribute buffer layouts. It has no feature flags, transport, archive
backend, compression dependency, runtime, or worker pool.

```rust
use i3s::{ResourceLocation, Source};

let source = Source::rest("https://example.com/SceneServer", 0);
let uri = ResourceLocation::NodePage(3).resolve(&source);
assert!(uri.ends_with("nodepages/3"));
```

## Application workflows

[Hiera](https://docs.rs/hiera) supplies resource acquisition, hierarchy
navigation, model conversion, editing, and package publication under
application-controlled execution and byte admission. Applications can depend
on Hiera alone:

```toml
[dependencies]
hiera = { version = "0.2", features = ["i3s"] }
```

`hiera::i3s` re-exports this crate's types and utilities alongside
`SceneLayerReader`, `SceneLayerHierarchy`, and the other adapters.
Its `SpecError` is `i3s::Error`; its `Error` describes adapter failures.
Enable Hiera's `moderu` feature for glTF model integration and `i3s-package`
for local SLPK/eSLPK storage. Neither is required for native document reads.

## Development

Generated modules are checked in; building the crate does not require the
schema submodule or generator. `generate.ps1` drives the development-only
`xtask` toolchain. Real-world fixture tests require `tests/realworld/`, which
is intentionally omitted from the published package.

## License

Apache License 2.0. See [LICENSE](LICENSE).
