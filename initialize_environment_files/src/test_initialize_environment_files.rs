#[test]
fn test_generation_creates_and_replaces_every_owned_file() {
    let workspace_root = std::env::temp_dir().join(env!("CARGO_PKG_NAME"));
    if workspace_root.exists() {
        assert!(matches!(
            std::fs::remove_dir_all(workspace_root.as_path()),
            Ok(())
        ));
    }
    assert!(matches!(
        std::fs::create_dir_all(workspace_root.join(constants_str::VALUE_B3EACD33)),
        Ok(())
    ));
    assert!(matches!(
        std::fs::create_dir_all(
            workspace_root.join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_CONFIG)
        ),
        Ok(())
    ));
    assert!(matches!(
        std::fs::write(
            workspace_root
                .join(constants_str::VALUE_B3EACD33)
                .join(constants_str::ENV),
            b"stale",
        ),
        Ok(())
    ));
    assert!(matches!(
        super::generate_environment_files::generate_environment_files(workspace_root.as_path()),
        Ok(())
    ));
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .into_iter()
        .collect::<Vec<_>>();
    let expected = |relative_path| {
        let path = repository_root
            .first()
            .map(|path| path.join(relative_path))?;
        std::fs::read(path).ok()
    };
    assert!(
        std::fs::read(
            workspace_root
                .join(constants_str::VALUE_B3EACD33)
                .join(constants_str::ENV)
        )
        .is_ok_and(|content| {
            expected(
                std::path::PathBuf::from(constants_str::VALUE_B3EACD33).join(constants_str::ENV),
            )
            .is_some_and(|expected_content| content == expected_content)
        })
    );
    assert!(
        std::fs::read(
            workspace_root
                .join(constants_str::VALUE_B3EACD33)
                .join(constants_str::ENV_EXAMPLE)
        )
        .is_ok_and(|content| {
            expected(
                std::path::PathBuf::from(constants_str::VALUE_B3EACD33).join(constants_str::ENV),
            )
            .is_some_and(|expected_content| content == expected_content)
        })
    );
    assert!(
        std::fs::read(workspace_root.join(constants_str::VALUE_0A7A2313)).is_ok_and(|content| {
            expected(std::path::PathBuf::from(constants_str::VALUE_0A7A2313))
                .is_some_and(|expected_content| content == expected_content)
        })
    );
    assert!(
        std::fs::read(
            workspace_root
                .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_CONFIG)
                .join(constants_str::ENV)
        )
        .is_ok_and(|content| {
            expected(std::path::PathBuf::from(constants_str::VALUE_0A7A2313))
                .is_some_and(|expected_content| content == expected_content)
        })
    );
    assert!(
        std::fs::read(workspace_root.join(constants_str::VALUE_E45E45BA)).is_ok_and(|content| {
            expected(std::path::PathBuf::from(constants_str::VALUE_E45E45BA))
                .is_some_and(|expected_content| content == expected_content)
        })
    );
    assert!(matches!(std::fs::remove_dir_all(workspace_root), Ok(())));
}

#[test]
fn test_configuration_field_preserves_text_and_replaces_invalid_utf8() {
    [
        (
            constants_str::EMPTY.as_bytes(),
            String::from(constants_str::EMPTY),
        ),
        (
            constants_str::JSON.as_bytes(),
            String::from(constants_str::JSON),
        ),
        (&[0xc3u8, 0xa9u8], char::from(233u8).to_string()),
        (&[0xffu8], char::REPLACEMENT_CHARACTER.to_string()),
    ]
    .into_iter()
    .fold((), |(), (bytes, expected)| {
        let field = crate::configuration_field::ConfigurationField::from(bytes);
        assert_eq!(field.value(), expected.as_str());
        let mut content = crate::std_byte_vector::StdByteVector::default();
        field.append_to(&mut content);
        assert_eq!(content.get_bounded_string().as_str(), expected.as_str());
    });
}

#[test]
fn test_environment_text_append_preserves_order_and_source() {
    let mut source = crate::std_byte_vector::StdByteVector::default();
    let mut destination = crate::std_byte_vector::StdByteVector::default();
    assert_eq!(source.get_bounded_string().as_str(), constants_str::EMPTY);
    crate::configuration_field::ConfigurationField::from(constants_str::JSON.as_bytes())
        .append_to(&mut source);
    crate::configuration_field::ConfigurationField::from(constants_str::X.as_bytes())
        .append_to(&mut destination);
    destination.append_std_byte_vector(&source);
    destination.append_std_byte_vector(&crate::std_byte_vector::StdByteVector::default());
    crate::configuration_field::ConfigurationField::from(constants_str::SPACE.as_bytes())
        .append_to(&mut destination);
    assert_eq!(
        destination.get_bounded_string().as_str(),
        format!(
            "{}{}{}",
            constants_str::X,
            constants_str::JSON,
            constants_str::SPACE
        ),
    );
    assert_eq!(source.get_bounded_string().as_str(), constants_str::JSON);
}

#[test]
fn test_server_environment_preserves_distinct_migrate_and_serve_orders() {
    let environment = crate::docker_compose_server_environment::DockerComposeServerEnvironment::new(
        crate::configuration_field::ConfigurationField::from(constants_str::JSON.as_bytes()),
        crate::configuration_field::ConfigurationField::from(constants_str::TRUE.as_bytes()),
        crate::configuration_field::ConfigurationField::from(constants_str::X.as_bytes()),
    );
    [
        (crate::docker_compose_server_environment_order::DockerComposeServerEnvironmentOrder::Migrate,
            format!("{}{}{}", constants_str::JSON, constants_str::X, constants_str::TRUE)),
        (crate::docker_compose_server_environment_order::DockerComposeServerEnvironmentOrder::Serve,
            format!("{}{}{}", constants_str::JSON, constants_str::TRUE, constants_str::X)),
    ]
    .into_iter()
    .fold((), |(), (order, expected)| {
        let content = environment.content(&order);
        assert_eq!(content.get_bounded_string().as_str(), expected.as_str());
    });
    assert_eq!(environment.get_database_url().value(), constants_str::JSON);
    assert_eq!(environment.get_service_mode().value(), constants_str::TRUE);
    assert_eq!(
        environment.get_service_socket_address().value(),
        constants_str::X
    );
}

#[test]
fn test_notification_environment_preserves_all_six_field_positions() {
    let field =
        |text: &'static str| crate::configuration_field::ConfigurationField::from(text.as_bytes());
    let environment = crate::docker_compose_notification_service_environment::DockerComposeNotificationServiceEnvironment::new(
        field(constants_str::JSON),
        field(constants_str::X),
        field(constants_str::TRUE),
        field(constants_str::SPACE),
        field(constants_str::FALSE),
        field(constants_str::TRACING_FORMAT_TEXT),
    );
    let content = environment.content();
    assert_eq!(
        content.get_bounded_string().as_str(),
        format!(
            "{}{}{}{}{}{}",
            constants_str::JSON,
            constants_str::X,
            constants_str::TRUE,
            constants_str::SPACE,
            constants_str::FALSE,
            constants_str::TRACING_FORMAT_TEXT
        ),
    );
}

#[test]
fn test_generation_propagates_missing_directory_errors_and_preserves_partial_output() {
    let workspace_root = std::env::temp_dir().join(stringify!(
        test_generation_propagates_missing_directory_errors_and_preserves_partial_output
    ));
    if workspace_root.exists() {
        assert!(matches!(std::fs::remove_dir_all(&workspace_root), Ok(())));
    }
    let fails_with_missing_directory = || {
        assert!(
            crate::generate_environment_files::generate_environment_files(&workspace_root)
                .is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound)
        );
    };
    fails_with_missing_directory();
    assert!(!workspace_root.exists());
    assert!(matches!(std::fs::create_dir_all(&workspace_root), Ok(())));
    fails_with_missing_directory();
    let compose_path = workspace_root.join(constants_str::VALUE_E45E45BA);
    let compose_content = std::fs::read(&compose_path);
    assert!(compose_content.is_ok());
    let Ok(expected_compose) = compose_content else {
        return;
    };
    assert!(!expected_compose.is_empty());
    assert!(!workspace_root.join(constants_str::VALUE_B3EACD33).exists());
    assert!(!workspace_root.join(constants_str::VALUE_0A7A2313).exists());
    assert!(
        !workspace_root
            .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_CONFIG)
            .exists()
    );
    assert!(matches!(
        std::fs::create_dir_all(workspace_root.join(constants_str::VALUE_B3EACD33)),
        Ok(())
    ));
    fails_with_missing_directory();
    assert!(std::fs::read(&compose_path).is_ok_and(|content| content == expected_compose));
    [constants_str::ENV, constants_str::ENV_EXAMPLE]
        .into_iter()
        .fold((), |(), filename| {
            assert!(
                std::fs::read(
                    workspace_root
                        .join(constants_str::VALUE_B3EACD33)
                        .join(filename)
                )
                .is_ok_and(|content| content
                    == server_config::server_config::ServerConfig::env_example().as_bytes())
            );
        });
    assert!(!workspace_root.join(constants_str::VALUE_0A7A2313).exists());
    assert!(
        !workspace_root
            .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_CONFIG)
            .exists()
    );
    assert!(matches!(std::fs::remove_dir_all(workspace_root), Ok(())));
}
