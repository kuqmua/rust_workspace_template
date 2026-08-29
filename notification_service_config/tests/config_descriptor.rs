// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(unused_crate_dependencies)] // integration target exercises generated descriptors while library-only dependencies remain linked by Cargo

#[cfg(test)]
mod tests {
    fn descriptor_examples() -> std::collections::BTreeMap<String, String> {
        notification_service_config::config::Config::env_example()
            .lines()
            .filter_map(|line| line.split_once('='))
            .map(|(name, value)| (name.to_owned(), value.to_owned()))
            .collect()
    }

    fn repository_file(path: &std::path::Path) -> String {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect("a884a7d7 repository_file invariant must hold")
                .join(path),
        )
        .expect("36a7831b repository_file invariant must hold")
    }

    #[test]
    #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
    fn env_example_matches_generated_descriptor() {
        let example_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(constants_str::catalog::ENV_EXAMPLE);
        if std::env::var_os(constants_str::test_fixtures::UPDATE_CONFIG_PROJECTIONS).is_some() {
            std::fs::write(
                example_path.as_path(),
                notification_service_config::config::Config::env_example(),
            )
            .expect("49f0c61e env_example_matches_generated_descriptor invariant must hold");
        }
        let example_source = std::fs::read_to_string(example_path)
            .expect("8db042aa env_example_matches_generated_descriptor invariant must hold");
        assert_eq!(
            example_source,
            notification_service_config::config::Config::env_example()
        );
        let examples = descriptor_examples();
        let descriptors = notification_service_config::config::Config::field_descriptors();
        assert_eq!(descriptors.len(), examples.len());
        descriptors.into_iter().for_each(|descriptor| {
            let value = examples
                .get(descriptor.env_name().as_ref())
                .cloned()
                .expect("00960401 env_example_matches_generated_descriptor invariant must hold");
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
                        config_lib::std_env_var_ok::StdEnvVarOk::try_from(value).expect(
                            "51cc2fcf env_example_matches_generated_descriptor invariant must hold"
                        )
                    ),
                    config_lib::config_example_validity::ConfigExampleValidity::Valid
                );
            }
        });
    }

    #[test]
    fn compose_environment_keys_match_generated_descriptor() {
        let compose_source = repository_file(std::path::Path::new(
            constants_str::test_fixtures::VALUE_E45E45BA,
        ));
        let service = compose_source
            .split_once(constants_str::test_fixtures::VALUE_3D732A3D)
            .map(|(_before, service)| service)
            .and_then(|service| {
                service
                    .split_once(constants_str::test_fixtures::VALUE_A71DB4E8)
                    .map(|(env, _after)| env)
            })
            .expect(
                "f30296b7 compose_environment_keys_match_generated_descriptor invariant must hold",
            );
        let environment = service
            .split_once(constants_str::test_fixtures::VALUE_22746334)
            .map(|(_before, environment)| environment)
            .expect(
                "0a7b014c compose_environment_keys_match_generated_descriptor invariant must hold",
            );
        let observed = environment
            .lines()
            .filter_map(|line| {
                line.trim()
                    .split_once(':')
                    .map(|(name, _value)| name.to_owned())
            })
            .collect::<std::collections::BTreeSet<_>>();
        let expected = notification_service_config::config::Config::field_descriptors()
            .into_iter()
            .map(|descriptor| descriptor.env_name().as_ref().to_owned())
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(observed, expected);
    }

    #[test]
    #[allow(clippy::needless_for_each)] // workspace source policy forbids for loops
    fn deployment_ports_match_generated_descriptor() {
        let mut socket_addresses = descriptor_examples()
            .into_iter()
            .filter(|(name, _value)| name.ends_with(constants_str::test_fixtures::VALUE_E0071B88))
            .map(|(_name, value)| value);
        let socket_address = socket_addresses
            .next()
            .expect("6ff25e0d deployment port descriptor must exist");
        assert!(socket_addresses.next().is_none(), "d11594d2");
        let port = socket_address
            .parse::<std::net::SocketAddr>()
            .expect("323370cc deployment port descriptor must contain a socket address")
            .port();
        let compose_source = repository_file(std::path::Path::new(
            constants_str::test_fixtures::VALUE_E45E45BA,
        ));
        [
            format!("NOTIFICATION_SERVICE_SOCKET_ADDRESS: \"0.0.0.0:{port}\""),
            format!("http://127.0.0.1:{port}/health/ready"),
            format!("\"127.0.0.1:{port}:{port}\""),
        ]
        .into_iter()
        .for_each(|expected| assert!(compose_source.contains(expected.as_str()), "0eea7688"));

        let deployment_source = repository_file(std::path::Path::new(
            constants_str::test_fixtures::VALUE_09101A6F,
        ));
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

        let network_policy_source = repository_file(std::path::Path::new(
            constants_str::test_fixtures::VALUE_7C89676C,
        ));
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
    fn service_image_references_follow_the_config_crate_name() {
        let service_name = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .and_then(|name| name.strip_suffix(constants_str::test_fixtures::VALUE_C48F2769))
            .map(str::to_owned)
            .expect("f53ffbf0 service name invariant must hold")
            .into_boxed_str();
        let dockerfile = std::path::PathBuf::from(service_name.as_ref())
            .join(constants_str::test_fixtures::VALUE_DD2C0EB6);
        assert!(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect("85dce7d5 service_image_references_follow_the_config_crate_name invariant must hold")
                .join(&dockerfile)
                .is_file(),
            "add133fd"
        );
        let dockerfile_text = dockerfile.to_str().expect(
            "abc73cd7 service_image_references_follow_the_config_crate_name invariant must hold",
        );
        let compose_source = repository_file(std::path::Path::new(
            constants_str::test_fixtures::VALUE_E45E45BA,
        ));
        assert!(
            compose_source.contains(format!("dockerfile: {dockerfile_text}").as_str()),
            "639c8124"
        );
        let ci_source = repository_file(std::path::Path::new(
            constants_str::catalog::CODE_STYLE_CI_WORKFLOW_PATH,
        ));
        assert!(
            ci_source.contains(format!("dockerfile: {dockerfile_text}").as_str()),
            "22e74268"
        );
        let release_source = repository_file(std::path::Path::new(
            constants_str::test_fixtures::VALUE_87DB21A9,
        ));
        assert!(
            release_source.contains(format!("dockerfile: {dockerfile_text}").as_str()),
            "f4cd7ec6"
        );
    }
}
