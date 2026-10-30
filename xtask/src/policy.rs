use heck::ToUpperCamelCase;
use schemagen::ir::SchemaNode;
use schemagen::{GenerationPolicy, StringRepr, TypeSettings};
use std::path::Path;

/// Types whose schema file stem is a poor name, by lowercased stem.
const NAME_OVERRIDES: &[(&str, &str)] = &[
    ("3dscenelayer", "SceneLayerInfo"),
    ("3dnodeindexdocument", "NodeIndexDocument"),
    ("sharedresource", "SharedResources"),
    ("attributestats", "AttributeStatistics"),
];

/// The rules for one I3S profile's module.
///
/// Data about particular schemas lives in `config.rs`; the representation
/// choices, the naming rule and the profile filter are code.
pub struct I3sPolicy<'a> {
    pub profile: &'a str,
}

impl GenerationPolicy for I3sPolicy<'_> {
    fn settings(&self) -> TypeSettings {
        // I3S documents are parsed once and read many times, so boxed strings
        // trade a growth capacity nobody uses for eight bytes per optional
        // field. Numeric widths stay at JSON's own, because the spec's own
        // `minimum`/`maximum` bounds already narrow the fields that can be
        // narrowed, and narrowing the rest by default would be a guess.
        TypeSettings::default().with_string(StringRepr::Boxed)
    }

    /// Names a type after its schema file, suffixed by its profile unless it
    /// is a common type.
    fn type_name(&self, title: &str, schema: &SchemaNode) -> Option<String> {
        let stem = Path::new(&schema.location.file)
            .file_stem()?
            .to_string_lossy()
            .strip_suffix(".schema")
            .unwrap_or_default()
            .to_string();
        let key = stem.to_lowercase();
        let base = NAME_OVERRIDES
            .iter()
            .find(|(known, _)| *known == key)
            .map(|(_, name)| (*name).to_owned())
            .or_else(|| (!stem.is_empty()).then(|| stem.to_upper_camel_case()))
            .or_else(|| (!title.is_empty()).then(|| title.to_upper_camel_case()))?;
        let schema_profile = Path::new(&schema.location.file)
            .parent()
            .and_then(Path::file_name)
            .map(|name| name.to_string_lossy().to_upper_camel_case());
        Some(match schema_profile.as_deref() {
            Some("Cmn") | None => base,
            Some(profile) => format!("{base}{profile}"),
        })
    }

    fn should_generate(&self, _title: &str, schema: &SchemaNode) -> bool {
        Path::new(&schema.location.file)
            .parent()
            .and_then(Path::file_name)
            .is_some_and(|profile| profile == self.profile)
    }
}
