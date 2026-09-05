use heck::ToUpperCamelCase;
use schemagen::ir::SchemaNode;
use schemagen::policy::GenerationPolicy;
use schemagen::settings::TypeSettings;
use schemagen::types::RustType;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct I3sConfig {
    #[serde(default)]
    pub name_overrides: std::collections::HashMap<String, String>,
}

pub struct I3sPolicy<'a> {
    pub profile: &'a str,
    pub config: &'a I3sConfig,
}

impl GenerationPolicy for I3sPolicy<'_> {
    fn settings(&self) -> TypeSettings {
        // I3S documents are parsed once and read many times, so boxed strings
        // trade a growth capacity nobody uses for eight bytes per optional
        // field. Numeric widths stay at JSON's own, because the spec's own
        // `minimum`/`maximum` bounds already narrow the fields that can be
        // narrowed, and narrowing the rest by default would be a guess.
        TypeSettings::default().with_string(schemagen::StringRepr::Boxed)
    }

    fn type_name(&self, title: &str, schema: &SchemaNode) -> Option<String> {
        let stem = Path::new(&schema.location.file)
            .file_stem()?
            .to_string_lossy()
            .strip_suffix(".schema")
            .unwrap_or_default()
            .to_string();
        let key = stem.to_lowercase();
        let base = self
            .config
            .name_overrides
            .get(&key)
            .cloned()
            .or_else(|| (!stem.is_empty()).then(|| pascal_ident(&stem)))
            .or_else(|| (!title.is_empty()).then(|| pascal_ident(title)))?;
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

    fn additional_definitions(&self) -> Vec<proc_macro2::TokenStream> {
        if self.profile == "cmn" {
            Vec::new()
        } else {
            vec![quote::quote!(
                use super::cmn::*;
            )]
        }
    }

    fn field_type(
        &self,
        _owner: &SchemaNode,
        field: &str,
        _schema: &SchemaNode,
    ) -> Option<RustType> {
        (field == "$ref").then_some(RustType::String)
    }
}

fn pascal_ident(value: &str) -> String {
    value.to_upper_camel_case()
}
