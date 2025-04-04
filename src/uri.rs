use crate::options::Compression;

pub trait UriBuilder {
    fn create_geometry_uri(
        &self,
        resource: &usize,
        compression: &Compression,
    ) -> Result<String, String>;
    fn create_texture_uri(
        &self,
        resource: &usize,
        name: &str,
        fmt: &str,
        compression: &Compression,
    ) -> Result<String, String>;
}
