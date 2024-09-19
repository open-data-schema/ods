use std::{fs::create_dir_all, path::PathBuf};

use clap::Parser;
use tracing::instrument;

use crate::{
    error::{Error, Result},
    schema::SchemaOpt,
};

/// Generate code from plan
#[derive(Debug, Parser)]
pub struct Generate {
    #[clap(flatten)]
    schema: SchemaOpt,

    #[clap(short)]
    output: Option<PathBuf>,
}

impl Generate {
    #[instrument(name = "gen", skip_all)]
    pub fn run(self) -> Result {
        let files = self.schema.load()?;

        let output = match self.output {
            Some(output) => output,
            // If the plan is a file, write the output to the parent folder
            None if self.schema.plan.is_file() => self
                .schema
                .plan
                .parent()
                .ok_or(Error::NoOutputDestination)?
                .join("ods"),
            None => self.schema.plan.join("ods"),
        };

        create_dir_all(&output)?;

        for (_name, _spec) in files {}

        Ok(())
    }
}
