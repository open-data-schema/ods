use clap::Parser;
use tracing::instrument;

use crate::error::Result;

/// Generate code
#[derive(Debug, Parser)]
pub struct Generate {}

impl Generate {
    #[instrument(name = "gen", skip_all)]
    pub fn run(self) -> Result {
        Ok(())
    }
}
