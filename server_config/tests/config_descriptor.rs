#![allow(
    unused_crate_dependencies,
    reason = "config descriptor requires this localized allowance for generated or framework-constrained code verified by focused tests"
)]

#[cfg(test)]
mod tests {
    #[test]
    #[allow(
        clippy::needless_for_each,
        reason = "config descriptor uses iterator traversal to comply with the workspace no-for-loop policy"
    )]
    fn test_env_example_matches_generated_config_descriptor_and_parsers() {
        let example_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(constants_str::SERVER_DOT_ENV_EXAMPLE);
        if std::env::var_os(constants_str::UPDATE_CONFIG_PROJECTIONS).is_some() {
            std::fs::write(
                example_path.as_path(),
                server_config::server_config::ServerConfig::env_example(),
            )
            .expect(constants_str::DIAGNOSTIC_C4A18F7D);
        }
        let example_source =
            std::fs::read_to_string(&example_path).expect(constants_str::DIAGNOSTIC_2A8737DD);
        assert_eq!(
            example_source,
            server_config::server_config::ServerConfig::env_example()
        );
        #[allow(
            deprecated,
            reason = "dotenv's iterator parses the runtime file format without mutating process environment shared by tests; its replacement writes global environment state"
        )]
        let examples = dotenv::from_path_iter(example_path)
            .expect(constants_str::DIAGNOSTIC_6EA47B6F)
            .collect::<Result<std::collections::BTreeMap<String, String>, _>>()
            .expect(constants_str::DIAGNOSTIC_7A548BC3);
        let descriptors = server_config::server_config::ServerConfig::field_descriptors();
        assert_eq!(descriptors.len(), examples.len());
        descriptors.into_iter().for_each(|descriptor| {
            let value = examples
                .get(descriptor.env_name().as_ref())
                .cloned()
                .expect(constants_str::DIAGNOSTIC_C8517AB3);
            assert_eq!(value, descriptor.example().as_ref());
            assert_eq!(
                descriptor.requirement(),
                config_lib::config_field_requirement::ConfigFieldRequirement::Required
            );
            if descriptor.sensitivity()
                == config_lib::config_field_sensitivity::ConfigFieldSensitivity::Public
            {
                assert_eq!(
                    descriptor.validate_example(
                        config_lib::std_env_var_ok::StdEnvVarOk::try_from(value)
                            .expect(constants_str::DIAGNOSTIC_92AE8A38)
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
