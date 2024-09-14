use clap::Parser;
use tracing::instrument;

use crate::{error::Result, schema::SchemaOpt};

#[cfg(test)]
mod tests;

/// Lint schema
#[derive(Debug, Parser)]
pub struct Lint {
    #[clap(flatten)]
    schema: SchemaOpt,
}

impl Lint {
    #[instrument(name = "lint", skip_all)]
    pub fn run(self) -> Result {
        let (files, _) = self.schema.load()?;

        for (name, spec) in files {}

        Ok(())
    }
}
