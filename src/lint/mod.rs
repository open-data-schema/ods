use anstream::println;
use clap::Parser;
use owo_colors::OwoColorize;
use proc_exit::Code;
use serde::Deserialize;
use tracing::{debug, instrument, trace};

use crate::{
    error::{exit, Result},
    lint::rules::Rules,
    schema::SchemaOpt,
};

pub mod rules;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
enum LintLevel {
    #[serde(rename = "off")]
    Off,

    #[serde(rename = "warn")]
    Warning,

    #[serde(rename = "error")]
    Error,
}

#[derive(Debug)]
struct LintResult {
    message: String,
}

/// Lint plan
#[derive(Debug, Parser)]
pub struct Lint {
    #[clap(flatten)]
    schema: SchemaOpt,

    /// Exit with a zero code even on lint errors
    #[clap(long)]
    no_fail: bool,
}

impl Lint {
    #[instrument(name = "lint", skip_all)]
    pub fn run(self) -> Result {
        let mut warnings = 0;
        let mut errors = 0;

        let files = self.schema.load()?;

        for (name, spec) in files {
            debug!("Linting spec: {}", name);
            let spec_results = Rules::run(&spec)?;

            if spec_results.is_empty() {
                trace!("No issues found in spec: {}", name);
                continue;
            }

            println!("\n{}", name.magenta());

            for (data_name, results) in spec_results {
                println!("  {}", data_name.cyan());

                for (level, result) in results {
                    match level {
                        LintLevel::Off => {}
                        LintLevel::Warning => {
                            warnings += 1;
                            println!("    {} {}", " warn".yellow(), result.message);
                        }
                        LintLevel::Error => {
                            errors += 1;
                            println!("    {} {}", "error".red(), result.message);
                        }
                    }
                }
            }
        }

        if warnings > 0 || errors > 0 {
            println!(
                "\n{} errors, {} warnings\n",
                errors.red().bold(),
                warnings.yellow().bold()
            );
        }

        if errors > 0 && !self.no_fail {
            exit(Code::FAILURE);
        }

        Ok(())
    }
}
