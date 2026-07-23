#![allow(unused_crate_dependencies)] // integration target exercises generated descriptors through server_config while library-only dependencies remain linked by Cargo

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
    fn env_example_matches_generated_config_descriptor_and_parsers() {
        let example_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(str_constants::SERVER_DOT_ENV_EXAMPLE);
        let example_source = std::fs::read_to_string(example_path).expect("2a8737dd");
        assert_eq!(example_source, server_config::Config::env_example());
        let examples = example_source
            .lines()
            .filter_map(|line| line.split_once('='))
            .map(|(name, value)| (name.to_owned(), value.to_owned()))
            .collect::<std::collections::BTreeMap<String, String>>();
        let descriptors = server_config::Config::field_descriptors();
        assert_eq!(descriptors.len(), examples.len());
        descriptors.into_iter().for_each(|descriptor| {
            let value = examples
                .get(descriptor.env_name().as_ref())
                .cloned()
                .expect("c8517ab3");
            if descriptor.sensitivity() == config_lib::ConfigFieldSensitivity::Public {
                assert_eq!(
                    descriptor.validate_example(
                        config_lib::StdEnvVarOk::try_from(value).expect("92ae8a38")
                    ),
                    config_lib::ConfigExampleValidity::Valid,
                    "{} {}",
                    descriptor.env_name().as_ref(),
                    descriptor.rust_type_name().as_ref()
                );
            }
        });
    }
}
