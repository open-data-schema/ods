macro_rules! rules {
    ($($rule:ident,)+) => {
        $(mod $rule;)+

        #[allow(non_camel_case_types)]
        #[derive(Clone, ValueEnum)]
        pub(super) enum Rules {
            $($rule,)+
        }

        impl std::fmt::Display for Rules {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(Rules::$rule => write!(f, stringify!($rule)),)+
                }
            }
        }

        paste::paste! {
            $(
                #[allow(non_camel_case_types)]
                #[derive(Debug, Clone, Default, Deserialize)]
                struct [<$rule _config>] {
                    level: Option<LintLevel>,
                    #[serde(flatten)]
                    config: $rule::Config,
                }
            )+

            #[derive(Debug, Clone, Default, Deserialize)]
            pub struct RulesConfig {
                $($rule: Option<[<$rule _config>]>,)+
            }
        }

        impl RulesConfig {
            pub(super) fn run_rule(&self, rule: &Rules, spec: &Spec) -> Result<(LintLevel, LintItem, Vec<(String, LintResult)>)> {
                match rule {
                    $(Rules::$rule => {
                        let config = self
                            .$rule
                            .as_ref()
                            .cloned()
                            .unwrap_or_default();

                        let level = config.level.unwrap_or_else(|| config.config.level());
                        let ty = config.config.ty();

                        if level == LintLevel::Off {
                            return Ok((level, ty, vec![]));
                        }

                        let results = config.config.run(spec)?;

                        return Ok((level, ty, results));
                    })+
                }
            }
        }
    };
}
