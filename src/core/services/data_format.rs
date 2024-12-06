use crate::core::definitions::{Error, Manifest};

pub trait DataFormatService {}

pub trait GetManifest: DataFormatService {
    fn get_manifest(&self, text: String) -> Result<Manifest, Error>;
}
