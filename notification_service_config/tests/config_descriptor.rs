#![allow(
    unused_crate_dependencies,
    reason = "config descriptor requires this localized allowance for generated or framework-constrained code verified by focused tests"
)]

#[cfg(test)]
mod tests {
    fn descriptor_examples() -> std::collections::BTreeMap<String, String> {
        notification_service_config::notification_service_config::NotificationServiceConfig::env_example()
            .lines()
            .filter_map(|line| line.split_once('='))
            .map(|(name, value)| (name.to_owned(), value.to_owned()))
            .collect()
    }

    fn repository_file(path: &std::path::Path) -> String {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect(constants_str::DIAGNOSTIC_A884A7D7)
                .join(path),
        )
        .expect(constants_str::DIAGNOSTIC_36A7831B)
    }

    #[test]
    #[allow(
        clippy::needless_for_each,
        reason = "config descriptor uses iterator traversal to comply with the workspace no-for-loop policy"
    )]
    fn test_env_example_matches_generated_descriptor() {
        let example_path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(constants_str::ENV_EXAMPLE);
        if std::env::var_os(constants_str::UPDATE_CONFIG_PROJECTIONS).is_some() {
            std::fs::write(
                example_path.as_path(),
                notification_service_config::notification_service_config::NotificationServiceConfig::env_example(),
            )
            .expect(constants_str::DIAGNOSTIC_49F0C61E);
        }
        let example_source =
            std::fs::read_to_string(example_path).expect(constants_str::DIAGNOSTIC_8DB042AA);
        assert_eq!(
            example_source,
            notification_service_config::notification_service_config::NotificationServiceConfig::env_example()
        );
        let examples = descriptor_examples();
        let descriptors = notification_service_config::notification_service_config::NotificationServiceConfig::field_descriptors();
        assert_eq!(descriptors.len(), examples.len());
        descriptors.into_iter().for_each(|descriptor| {
            let value = examples
                .get(descriptor.env_name().as_ref())
                .cloned()
                .expect(constants_str::DIAGNOSTIC_00960401);
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
                            .expect(constants_str::DIAGNOSTIC_51CC2FCF)
                    ),
                    config_lib::config_example_validity::ConfigExampleValidity::Valid
                );
            }
        });
    }

    #[test]
    fn test_compose_environment_keys_match_generated_descriptor() {
        let compose_source = repository_file(std::path::Path::new(constants_str::VALUE_E45E45BA));
        let service = compose_source
            .split_once(constants_str::VALUE_3D732A3D)
            .map(|(_before, service)| service)
            .and_then(|service| {
                service
                    .split_once(constants_str::VALUE_A71DB4E8)
                    .map(|(env, _after)| env)
            })
            .expect(constants_str::DIAGNOSTIC_F30296B7);
        let environment = service
            .split_once(constants_str::VALUE_22746334)
            .map(|(_before, environment)| environment)
            .expect(constants_str::DIAGNOSTIC_0A7B014C);
        let observed = environment
            .lines()
            .filter_map(|line| {
                line.trim()
                    .split_once(':')
                    .map(|(name, _value)| name.to_owned())
            })
            .collect::<std::collections::BTreeSet<_>>();
        let expected = notification_service_config::notification_service_config::NotificationServiceConfig::field_descriptors()
            .into_iter()
            .map(|descriptor| descriptor.env_name().as_ref().to_owned())
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(observed, expected);
    }

    #[test]
    #[allow(
        clippy::needless_for_each,
        reason = "config descriptor uses iterator traversal to comply with the workspace no-for-loop policy"
    )]
    fn test_deployment_ports_match_generated_descriptor() {
        let mut socket_addresses = descriptor_examples()
            .into_iter()
            .filter(|(name, _value)| name.ends_with(constants_str::VALUE_E0071B88))
            .map(|(_name, value)| value);
        let socket_address = socket_addresses
            .next()
            .expect(constants_str::DIAGNOSTIC_6FF25E0D);
        assert!(socket_addresses.next().is_none(), "d11594d2");
        let port = socket_address
            .parse::<std::net::SocketAddr>()
            .expect(constants_str::DIAGNOSTIC_323370CC)
            .port();
        let compose_source = repository_file(std::path::Path::new(constants_str::VALUE_E45E45BA));
        [
            format!("NOTIFICATION_SERVICE_SOCKET_ADDRESS: \"0.0.0.0:{port}\""),
            format!("http://127.0.0.1:{port}/health/ready"),
            format!("\"127.0.0.1:{port}:{port}\""),
        ]
        .into_iter()
        .for_each(|expected| assert!(compose_source.contains(expected.as_str()), "0eea7688"));

        let deployment_source =
            repository_file(std::path::Path::new(constants_str::VALUE_09101A6F));
        assert_eq!(
            deployment_source
                .matches(format!("containerPort: {port}").as_str())
                .count(),
            1usize,
            "af94f90b"
        );
        assert_eq!(
            deployment_source
                .lines()
                .filter(|line| line.trim() == format!("port: {port}"))
                .count(),
            1usize,
            "79fb8442"
        );

        let network_policy_source =
            repository_file(std::path::Path::new(constants_str::VALUE_7C89676C));
        assert_eq!(
            network_policy_source
                .lines()
                .filter(|line| line.trim() == format!("port: {port}"))
                .count(),
            2usize,
            "d0acf16a"
        );
    }

    #[test]
    fn test_service_image_references_follow_the_config_crate_name() {
        let service_name = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .and_then(|name| name.strip_suffix(constants_str::VALUE_C48F2769))
            .map(str::to_owned)
            .expect(constants_str::DIAGNOSTIC_F53FFBF0)
            .into_boxed_str();
        let dockerfile =
            std::path::PathBuf::from(service_name.as_ref()).join(constants_str::VALUE_DD2C0EB6);
        assert!(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect(constants_str::DIAGNOSTIC_85DCE7D5)
                .join(&dockerfile)
                .is_file(),
            "add133fd"
        );
        let dockerfile_text = dockerfile
            .to_str()
            .expect(constants_str::DIAGNOSTIC_ABC73CD7);
        let compose_source = repository_file(std::path::Path::new(constants_str::VALUE_E45E45BA));
        assert!(
            compose_source.contains(format!("dockerfile: {dockerfile_text}").as_str()),
            "639c8124"
        );
        let ci_source = repository_file(std::path::Path::new(
            constants_str::CODE_STYLE_CI_WORKFLOW_PATH,
        ));
        assert!(
            ci_source.contains(format!("dockerfile: {dockerfile_text}").as_str()),
            "22e74268"
        );
        let release_source = repository_file(std::path::Path::new(constants_str::VALUE_87DB21A9));
        assert!(
            release_source.contains(format!("dockerfile: {dockerfile_text}").as_str()),
            "f4cd7ec6"
        );
    }
}
