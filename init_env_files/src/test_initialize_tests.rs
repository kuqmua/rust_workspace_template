fn fixture() -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!(
        "rust-workspace-template-environment-{}",
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(root.join(constants_str::SERVICE))
        .expect(constants_str::DIAGNOSTIC_FDBF7411);
    std::fs::write(
        root.join(constants_str::CARGO_TOML),
        constants_str::WORKSPACE_NEWLINE_MEMBERS_SERVICE_NEWLINE,
    )
    .expect(constants_str::DIAGNOSTIC_8E781C83);
    std::fs::write(
        root.join(constants_str::SERVICE_ENV_EXAMPLE),
        constants_str::PUBLIC_VALUE_NEWLINE_SECRET_CHANGE_ME_NEWLINE,
    )
    .expect(constants_str::DIAGNOSTIC_F24FCA72);
    root
}
#[test]
fn test_dry_run_apply_and_repeat_are_safe_and_idempotent() {
    let root = fixture();
    let dry = crate::initialize::initialize(
        crate::workspace_root_path_ref::WorkspaceRootPathRef::from(root.as_path()),
        crate::run_mode::RunMode::DryRun,
    )
    .expect(constants_str::DIAGNOSTIC_93CE4136);
    assert_eq!(
        dry.as_ref()
            .first()
            .expect(constants_str::DIAGNOSTIC_14B080CA)
            .status(),
        crate::initialization_status::InitializationStatus::WouldCreate
    );
    assert!(!root.join(constants_str::SERVICE_ENV).exists());
    let applied = crate::initialize::initialize(
        crate::workspace_root_path_ref::WorkspaceRootPathRef::from(root.as_path()),
        crate::run_mode::RunMode::Apply,
    )
    .expect(constants_str::DIAGNOSTIC_D58ED6A5);
    assert_eq!(
        applied
            .as_ref()
            .first()
            .expect(constants_str::DIAGNOSTIC_C366CC59)
            .status(),
        crate::initialization_status::InitializationStatus::Created
    );
    std::fs::write(
        root.join(constants_str::SERVICE_ENV),
        constants_str::SECRET_CUSTOM_NEWLINE,
    )
    .expect(constants_str::DIAGNOSTIC_2D67B058);
    let updated = crate::initialize::initialize(
        crate::workspace_root_path_ref::WorkspaceRootPathRef::from(root.as_path()),
        crate::run_mode::RunMode::Apply,
    )
    .expect(constants_str::DIAGNOSTIC_546AF7B6);
    assert_eq!(
        updated
            .as_ref()
            .first()
            .expect(constants_str::DIAGNOSTIC_195600EC)
            .status(),
        crate::initialization_status::InitializationStatus::Updated
    );
    let updated_content = std::fs::read_to_string(root.join(constants_str::SERVICE_ENV))
        .expect(constants_str::DIAGNOSTIC_BD9F5208);
    assert!(updated_content.contains(constants_str::VALUE_F9629C76));
    assert!(updated_content.contains(constants_str::VALUE_E120A6D3));
    let repeated = crate::initialize::initialize(
        crate::workspace_root_path_ref::WorkspaceRootPathRef::from(root.as_path()),
        crate::run_mode::RunMode::Apply,
    )
    .expect(constants_str::DIAGNOSTIC_A452843A);
    assert_eq!(
        repeated
            .as_ref()
            .first()
            .expect(constants_str::DIAGNOSTIC_37A0752C)
            .status(),
        crate::initialization_status::InitializationStatus::SkippedExisting
    );
    std::fs::remove_dir_all(root).expect(constants_str::DIAGNOSTIC_BD9180CA);
}
#[test]
fn test_escaping_member_is_rejected() {
    let root = fixture();
    std::fs::write(
        root.join(constants_str::CARGO_TOML),
        constants_str::WORKSPACE_NEWLINE_MEMBERS_OUTSIDE_NEWLINE,
    )
    .expect(constants_str::DIAGNOSTIC_350646F2);
    assert!(matches!(
        crate::initialize::initialize(
            crate::workspace_root_path_ref::WorkspaceRootPathRef::from(root.as_path()),
            crate::run_mode::RunMode::DryRun
        ),
        Err(crate::initialize_error::InitializeError::InvalidMember { .. })
    ));
    std::fs::remove_dir_all(root).expect(constants_str::DIAGNOSTIC_D9154402);
}
#[test]
fn test_oversized_environment_example_is_rejected() {
    let root = fixture();
    std::fs::write(
        root.join(constants_str::SERVICE_ENV_EXAMPLE),
        constants_str::A_ALT
            .repeat(constants_usize::VALUE_1_048_576.saturating_add(constants_usize::ONE)),
    )
    .expect(constants_str::DIAGNOSTIC_F6290E85);
    assert!(matches!(
        crate::initialize::initialize(
            crate::workspace_root_path_ref::WorkspaceRootPathRef::from(root.as_path()),
            crate::run_mode::RunMode::DryRun
        ),
        Err(crate::initialize_error::InitializeError::ReadExample {
            source: server_runtime_http::bounded_read_error::BoundedReadError::ExceedsMaximum { .. }
        })
    ));
    std::fs::remove_dir_all(root).expect(constants_str::DIAGNOSTIC_7D83384C);
}
