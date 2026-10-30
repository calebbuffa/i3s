//! The decisions about the I3S schemas that no general rule predicts.

use schemagen::Config;

/// Builds the config for one profile's module.
///
/// Every profile but `cmn` re-exports nothing of its own and borrows the
/// common types, so they import them.
pub fn config(profile: &str) -> schemagen::Result<Config> {
    let builder = Config::builder()
        // The spec's `$ref` members are URIs, not schema references.
        .field_type("*.$ref", "String")
        .enum_name("HeaderValue.valueType", "HeaderValueType")
        // Esri writers emit `FeatureData`, the spec says `featureData`.
        .enum_alias("*.resourcePattern", "featureData", ["FeatureData"])
        // The spec marks these required, yet real packages list only the
        // attributes they carry.
        .schema("vertexAttribute", |s| {
            s.property_required("color", false)
                .property_required("normal", false)
                .property_required("uv0", false)
        });
    let builder = if profile == "cmn" {
        builder
    } else {
        builder.import("super::cmn::*")
    };
    builder.build()
}
