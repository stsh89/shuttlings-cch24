use crate::core::{
    definitions::{Error, Manifest},
    services::{DataFormatService, GetManifest},
};
use eyre::Report;
use std::str::FromStr;

pub struct TomlService {}

impl DataFormatService for TomlService {}

impl GetManifest for TomlService {
    fn get_manifest(&self, text: String) -> Result<Manifest, Error> {
        Manifest::from_str(&text)
            .map_err(Report::new)
            .map_err(Error::corrupted_data_format)
    }
}
