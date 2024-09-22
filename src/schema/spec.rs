use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use eyre::eyre;
use serde::Deserialize;

use crate::{error::Result, lint::rules::RulesConfig};

#[derive(Debug, Clone, Deserialize)]
pub struct Source {
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub description: Option<String>,
    pub sources: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    pub metrics: HashMap<String, Event>,
    pub sources: Option<HashMap<String, Source>>,
    pub lint: Option<RulesConfig>,
}

impl Spec {
    pub fn load(path: &PathBuf) -> Result<Spec> {
        let file_content = read_to_string(path)?;

        let spec = match path.extension().map(|v| v.to_string_lossy()) {
            Some(v) if v == "yaml" || v == "yml" => serde_yml::from_str(&file_content)?,
            Some(v) if v == "json" => serde_json::from_str(&file_content)?,
            _ => return Err(eyre!("unable to recognize file format")),
        };

        Ok(spec)
    }
}
