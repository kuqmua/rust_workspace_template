// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(unused_crate_dependencies)] // integration target exercises generated descriptors through server_config while library-only dependencies remain linked by Cargo

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
    fn env_example_matches_generated_config_descriptor_and_parsers() {
        let example_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(constants_str::test_fixtures::SERVER_DOT_ENV_EXAMPLE);
        if std::env::var_os(constants_str::test_fixtures::UPDATE_CONFIG_PROJECTIONS).is_some() {
            std::fs::write(example_path.as_path(), server_config::config::Config::env_example())
                .expect("c4a18f7d env_example_matches_generated_config_descriptor_and_parsers invariant must hold");
        }
        let example_source = std::fs::read_to_string(example_path).expect("2a8737dd env_example_matches_generated_config_descriptor_and_parsers invariant must hold");
        assert_eq!(example_source, server_config::config::Config::env_example());
        let examples = example_source
            .lines()
            .filter_map(|line| line.split_once('='))
            .map(|(name, value)| (name.to_owned(), value.to_owned()))
            .collect::<std::collections::BTreeMap<String, String>>();
        let descriptors = server_config::config::Config::field_descriptors();
        assert_eq!(descriptors.len(), examples.len());
        descriptors.into_iter().for_each(|descriptor| {
            let value = examples
                .get(descriptor.env_name().as_ref())
                .cloned()
                .expect("c8517ab3 env_example_matches_generated_config_descriptor_and_parsers invariant must hold");
            assert_eq!(value, descriptor.example().as_ref());
            assert_eq!(
                descriptor.requirement(),
                config_lib::config_field_requirement::ConfigFieldRequirement::Required
            );
            if descriptor.sensitivity() == config_lib::config_field_sensitivity::ConfigFieldSensitivity::Public {
                assert_eq!(
                    descriptor.validate_example(
                        config_lib::std_env_var_ok::StdEnvVarOk::try_from(value).expect("92ae8a38 env_example_matches_generated_config_descriptor_and_parsers invariant must hold")
                    ),
                    config_lib::config_example_validity::ConfigExampleValidity::Valid,
                    "{} {}",
                    descriptor.env_name().as_ref(),
                    descriptor.rust_type_name().as_ref()
                );
            }
        });
    }
}
