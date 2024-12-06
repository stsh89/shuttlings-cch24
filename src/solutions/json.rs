use crate::core::{
    definitions::{Error, Manifest},
    services::{DataFormatService, GetManifest},
};
use eyre::Report;

pub struct JsonService {}

impl DataFormatService for JsonService {}

impl GetManifest for JsonService {
    fn get_manifest(&self, text: String) -> Result<Manifest, Error> {
        serde_json::from_str(&text)
            .map_err(Report::new)
            .map_err(Error::corrupted_data_format)
    }
}
