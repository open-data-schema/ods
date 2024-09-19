use std::path::{PathBuf, MAIN_SEPARATOR};

use clap::Parser;
use indexmap::IndexMap;
use tracing::{debug, instrument, trace};

use crate::{error::Result, schema::spec::Spec};

pub mod spec;

#[derive(Debug, Parser)]
pub struct SchemaOpt {
    pub plan: PathBuf,
}

impl SchemaOpt {
    #[instrument(name = "load", skip_all)]
    pub fn load(&self) -> Result<IndexMap<String, Spec>> {
        let mut files = IndexMap::new();
        let base_path = format!("{}{}", self.plan.to_string_lossy(), MAIN_SEPARATOR);

        if self.plan.is_file() {
            trace!("Loading plan from specified file");
            files.insert(
                self.plan.file_name().unwrap().to_string_lossy().to_string(),
                Spec::load(&self.plan)?,
            );
        } else {
            trace!("Loading plans from specified folder");
            load_dir(&mut files, &self.plan, &base_path)?;
        }

        debug!("Loaded plans: {:#?}", files.len());
        Ok(files)
    }
}

fn load_dir(files: &mut IndexMap<String, Spec>, path: &PathBuf, base_path: &str) -> Result<()> {
    for entry in path.read_dir()? {
        let path = entry?.path();

        if path.is_file() {
            let relative_path = path.to_string_lossy().replace(base_path, "");

            trace!("Loading plan from file: {}", relative_path);
            files.insert(relative_path, Spec::load(&path)?);
        } else {
            trace!("Loading plans from folder: {}", path.to_string_lossy());
            load_dir(files, &path, base_path)?;
        }
    }

    Ok(())
}
