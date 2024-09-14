use std::{
    collections::HashMap,
    fs::{metadata, Metadata},
    path::PathBuf,
};

use clap::Parser;

use crate::{error::Result, schema::spec::Spec};

pub mod spec;

#[derive(Debug, Parser)]
pub struct SchemaOpt {
    pub plan: PathBuf,
}

impl SchemaOpt {
    pub fn load(&self) -> Result<(HashMap<String, Spec>, Metadata)> {
        let mut files = HashMap::new();
        let metadata = metadata(&self.plan)?;

        if metadata.is_file() {
            files.insert(String::from("plan"), Spec::load(&self.plan)?);
        } else {
            // Read all the files in the folder
        }

        Ok((files, metadata))
    }
}
