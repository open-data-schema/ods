use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use serde::Deserialize;

use crate::error::{Error, Result};

#[derive(Debug, Deserialize)]
struct Event {
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    events: HashMap<String, Event>,
}

impl Spec {
    pub fn load(path: &PathBuf) -> Result<Spec> {
        let file_content = read_to_string(path)?;

        let spec = match path.extension().map(|v| v.to_string_lossy()) {
            Some(v) if v == "yaml" || v == "yml" => serde_yml::from_str(&file_content)?,
            Some(v) if v == "json" => serde_json::from_str(&file_content)?,
            _ => return Err(Error::BadFileFormat),
        };

        Ok(spec)
    }
}
