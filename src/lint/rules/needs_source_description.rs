use serde::Deserialize;

use crate::{
    error::Result,
    lint::{rules::Rule, LintItem, LintResult},
    schema::spec::Spec,
};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {}

impl Rule for Config {
    fn ty(&self) -> LintItem {
        LintItem::Source
    }

    fn run(&self, spec: &Spec) -> Result<Vec<(String, LintResult)>> {
        let mut results = vec![];

        if let Some(sources) = spec.sources.as_ref() {
            for (name, source) in sources {
                if source.description.is_none() {
                    results.push((
                        name.clone(),
                        LintResult {
                            message: "description is missing".to_string(),
                        },
                    ));
                }
            }
        }

        Ok(results)
    }
}
