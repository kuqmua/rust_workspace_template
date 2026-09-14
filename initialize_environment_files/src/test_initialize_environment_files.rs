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
