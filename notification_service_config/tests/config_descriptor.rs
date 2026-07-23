#![allow(unused_crate_dependencies)] // integration target exercises generated descriptors while library-only dependencies remain linked by Cargo

#[cfg(test)]
mod tests {
    fn descriptor_examples() -> std::collections::BTreeMap<String, String> {
        notification_service_config::Config::env_example()
            .lines()
            .filter_map(|line| line.split_once('='))
            .map(|(name, value)| (name.to_owned(), value.to_owned()))
            .collect()
    }

    #[allow(clippy::single_call_fn)] // isolates descriptor-derived port resolution from deployment assertions
    fn descriptor_service_port() -> u16 {
        let mut socket_addresses = descriptor_examples()
            .into_iter()
            .filter(|(name, _value)| name.ends_with("_SERVICE_SOCKET_ADDRESS"))
            .map(|(_name, value)| value);
        let socket_address = socket_addresses.next().expect("6ff25e0d");
        assert!(socket_addresses.next().is_none(), "d11594d2");
        socket_address
            .parse::<std::net::SocketAddr>()
            .expect("323370cc")
            .port()
    }

    fn repository_file(path: &std::path::Path) -> String {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect("a884a7d7")
                .join(path),
        )
        .expect("36a7831b")
    }

    #[allow(clippy::single_call_fn)] // derives deployment identity from the config crate instead of repeating it
    fn service_name() -> Box<str> {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .and_then(|name| name.strip_suffix("_config"))
            .map(str::to_owned)
            .expect("f53ffbf0")
            .into_boxed_str()
    }

    #[test]
    #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
    fn env_example_matches_generated_descriptor() {
        let example_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env.example");
        let example_source = std::fs::read_to_string(example_path).expect("8db042aa");
        assert_eq!(
            example_source,
            notification_service_config::Config::env_example()
        );
        let examples = descriptor_examples();
        let descriptors = notification_service_config::Config::field_descriptors();
        assert_eq!(descriptors.len(), examples.len());
        descriptors.into_iter().for_each(|descriptor| {
            let value = examples
                .get(descriptor.env_name().as_ref())
                .cloned()
                .expect("00960401");
            if descriptor.sensitivity() == config_lib::ConfigFieldSensitivity::Public {
                assert_eq!(
                    descriptor.validate_example(
                        config_lib::StdEnvVarOk::try_from(value).expect("51cc2fcf")
                    ),
                    config_lib::ConfigExampleValidity::Valid
                );
            }
        });
    }

    #[test]
    fn compose_environment_keys_match_generated_descriptor() {
        let compose_source = repository_file(std::path::Path::new("docker-compose.yml"));
        let service = compose_source
            .split_once("  notification_service:\n")
            .map(|(_before, service)| service)
            .and_then(|service| {
                service
                    .split_once("    healthcheck:\n")
                    .map(|(env, _after)| env)
            })
            .expect("f30296b7");
        let environment = service
            .split_once("    environment:\n")
            .map(|(_before, environment)| environment)
            .expect("0a7b014c");
        let observed = environment
            .lines()
            .filter_map(|line| {
                line.trim()
                    .split_once(':')
                    .map(|(name, _value)| name.to_owned())
            })
            .collect::<std::collections::BTreeSet<_>>();
        let expected = notification_service_config::Config::field_descriptors()
            .into_iter()
            .map(|descriptor| descriptor.env_name().as_ref().to_owned())
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(observed, expected);
    }

    #[test]
    #[allow(clippy::needless_for_each)] // workspace source policy forbids for loops
    fn deployment_ports_match_generated_descriptor() {
        let port = descriptor_service_port();
        let compose_source = repository_file(std::path::Path::new("docker-compose.yml"));
        [
            format!("NOTIFICATION_SERVICE_SOCKET_ADDRESS: \"0.0.0.0:{port}\""),
            format!("http://127.0.0.1:{port}/health/ready"),
            format!("\"127.0.0.1:{port}:{port}\""),
        ]
        .into_iter()
        .for_each(|expected| assert!(compose_source.contains(expected.as_str()), "0eea7688"));

        let deployment_source = repository_file(std::path::Path::new(
            "deploy/k8s/base/notification-service.yaml",
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
            "deploy/k8s/base/network-policies.yaml",
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
        let service_name = service_name();
        let dockerfile = std::path::PathBuf::from(service_name.as_ref()).join("Dockerfile");
        assert!(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect("85dce7d5")
                .join(&dockerfile)
                .is_file(),
            "add133fd"
        );
        let dockerfile_text = dockerfile.to_str().expect("abc73cd7");
        let compose_source = repository_file(std::path::Path::new("docker-compose.yml"));
        assert!(
            compose_source.contains(format!("dockerfile: {dockerfile_text}").as_str()),
            "639c8124"
        );
        let ci_source = repository_file(std::path::Path::new(".github/workflows/ci.yml"));
        assert!(
            ci_source.contains(format!("--file {dockerfile_text}").as_str()),
            "22e74268"
        );
        let release_source = repository_file(std::path::Path::new(".github/workflows/release.yml"));
        assert!(
            release_source.contains(format!("dockerfile: {dockerfile_text}").as_str()),
            "f4cd7ec6"
        );
    }
}
