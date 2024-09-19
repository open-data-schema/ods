use std::{collections::HashMap, fmt::Debug as FmtDebug};

use clap::ValueEnum;
use serde::Deserialize;
use tracing::{instrument, trace};

use crate::{
    error::Result,
    lint::{LintLevel, LintResult},
    schema::spec::Spec,
};

#[macro_use]
mod macro_def;

rules! {
    needs_description,
    needs_explicit_sources,
}

impl Rules {
    // TODO: Let the user configure the rules for all specs in a single file
    // and merge them with the spec-specific rules
    #[instrument(name = "run", skip_all)]
    pub(super) fn run(spec: &Spec) -> Result<HashMap<String, Vec<(LintLevel, LintResult)>>> {
        let mut all_results = HashMap::new();
        let rules_config = spec.lint.as_ref().cloned().unwrap_or_default();

        for rule in Rules::value_variants() {
            trace!("Running rule: {}", rule);
            let (level, results) = rules_config.run_rule(&rule, spec)?;

            for (data_name, result) in results {
                all_results
                    .entry(data_name)
                    .or_insert_with(Vec::new)
                    .push((level.clone(), result));
            }
        }

        Ok(all_results)
    }
}

trait Rule: FmtDebug + Clone + Default + for<'de> Deserialize<'de> {
    fn level(&self) -> LintLevel {
        LintLevel::Off
    }

    fn run(&self, spec: &Spec) -> Result<Vec<(String, LintResult)>>;
}
