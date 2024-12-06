use crate::core::{
    definitions::{Error, Manifest},
    services::{DataFormatService, GetManifest},
};
use eyre::Report;

pub struct YamlService {}

impl DataFormatService for YamlService {}

impl GetManifest for YamlService {
    fn get_manifest(&self, text: String) -> Result<Manifest, Error> {
        serde_yml::from_str(&text)
            .map_err(Report::new)
            .map_err(Error::corrupted_data_format)
    }
}
