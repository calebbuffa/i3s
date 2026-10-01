# i3s

Reading Esri [Indexed 3D Scene Layers](https://github.com/Esri/i3s-spec) (I3S)
in Rust.

The document types in `i3s::generated` are produced from the I3S
specification's own markdown, so they track the specification rather than a
hand transcription of it. Around them this crate adds the parts a client needs
but the schema cannot express:

- **`Source`** - resource naming for REST services, `.slpk` and `.eslpk`, which differ in more than just a prefix.
- **Wire codecs** - the I3S geometry and attribute binary layouts, including
  the legacy 1.6 and node-page 1.7+ forms.
- **`SceneLayerLoader`** - demand-driven hierarchy discovery and recursively-addressed Building sublayers.
- **`SceneLayerReader`** - a typed reader that decorates a loader, retaining
  all `hiera::Loader` functionality while adding direct typed document and
  resource reads.
- **`SceneLayerNavigator`** - retainable, path-aware cursors over a layer.
- **`SceneLayerStore`** - caller-owned resource transport.

## Transport and threading belong to the caller

The crate performs no I/O of its own. A loader is given a fetch callback, and
traversal is driven by the application, so the same code works over HTTP, a
filesystem, an archive or a cache.

```rust
use i3s::{SceneLayerLoader, SceneLayerNavigator, SceneLayerReader, Source};
use std::sync::Arc;

let source = Source::rest("https://example.com/SceneServer", 0);
let loader = SceneLayerLoader::open(source, |request: hiera::FetchRequest| async move {
    // Your transport here.
    todo!()
});

let navigator = SceneLayerNavigator::new(SceneLayerReader::new(Arc::new(loader)));
let root = navigator.root().await?;
for child in root.children().await? {
    println!("{}", child.id());
}
```

## Licence

Apache License 2.0, see [LICENSE](LICENSE) for details.
