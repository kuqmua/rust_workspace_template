fn fixture() -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!(
        "rust-workspace-template-environment-{}",
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(root.join(constants_str::SERVICE))
        .expect("fdbf7411 fixture invariant must hold");
    std::fs::write(
        root.join(constants_str::CARGO_TOML),
        constants_str::WORKSPACE_NEWLINE_MEMBERS_SERVICE_NEWLINE,
    )
    .expect("8e781c83 fixture invariant must hold");
    std::fs::write(
        root.join(constants_str::SERVICE_ENV_EXAMPLE),
        constants_str::PUBLIC_VALUE_NEWLINE_SECRET_CHANGE_ME_NEWLINE,
    )
    .expect("f24fca72 fixture invariant must hold");
    root
}
#[test]
fn dry_run_apply_and_repeat_are_safe_and_idempotent() {
    let root = fixture();
    let dry = crate::initialize(
        crate::WorkspaceRootPathRef::from(root.as_path()),
        crate::RunMode::DryRun,
    )
    .expect("93ce4136 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
    assert_eq!(
        dry.0
            .first()
            .expect("14b080ca dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold")
            .status,
        crate::InitializationStatus::WouldCreate
    );
    assert!(!root.join("service/.env").exists());
    let applied = crate::initialize(
        crate::WorkspaceRootPathRef::from(root.as_path()),
        crate::RunMode::Apply,
    )
    .expect("d58ed6a5 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
    assert_eq!(
        applied
            .0
            .first()
            .expect("c366cc59 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold")
            .status,
        crate::InitializationStatus::Created
    );
    std::fs::write(
        root.join(constants_str::SERVICE_ENV),
        constants_str::SECRET_CUSTOM_NEWLINE,
    )
    .expect("2d67b058 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
    let updated = crate::initialize(
        crate::WorkspaceRootPathRef::from(root.as_path()),
        crate::RunMode::Apply,
    )
    .expect("546af7b6 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
    assert_eq!(
        updated
            .0
            .first()
            .expect("195600ec dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold")
            .status,
        crate::InitializationStatus::Updated
    );
    let updated_content = std::fs::read_to_string(root.join(constants_str::SERVICE_ENV))
        .expect("bd9f5208 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
    assert!(updated_content.contains("SECRET=custom"));
    assert!(updated_content.contains("PUBLIC=value"));
    let repeated = crate::initialize(
        crate::WorkspaceRootPathRef::from(root.as_path()),
        crate::RunMode::Apply,
    )
    .expect("a452843a dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
    assert_eq!(
        repeated
            .0
            .first()
            .expect("37a0752c dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold")
            .status,
        crate::InitializationStatus::SkippedExisting
    );
    std::fs::remove_dir_all(root)
        .expect("bd9180ca dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
}
#[test]
fn escaping_member_is_rejected() {
    let root = fixture();
    std::fs::write(
        root.join(constants_str::CARGO_TOML),
        constants_str::WORKSPACE_NEWLINE_MEMBERS_OUTSIDE_NEWLINE,
    )
    .expect("350646f2 escaping_member_is_rejected invariant must hold");
    assert!(matches!(
        crate::initialize(
            crate::WorkspaceRootPathRef::from(root.as_path()),
            crate::RunMode::DryRun
        ),
        Err(crate::InitializeError::InvalidMember { .. })
    ));
    std::fs::remove_dir_all(root)
        .expect("d9154402 escaping_member_is_rejected invariant must hold");
}
#[test]
fn oversized_environment_example_is_rejected() {
    let root = fixture();
    std::fs::write(
        root.join(constants_str::SERVICE_ENV_EXAMPLE),
        constants_str::A_ALT
            .repeat(constants_usize::VALUE_1_048_576.saturating_add(constants_usize::ONE)),
    )
    .expect("f6290e85 oversized_environment_example_is_rejected invariant must hold");
    assert!(matches!(
        crate::initialize(
            crate::WorkspaceRootPathRef::from(root.as_path()),
            crate::RunMode::DryRun
        ),
        Err(crate::InitializeError::ReadExample {
            source: server_runtime_http::domain_types::BoundedReadError::ExceedsMaximum { .. }
        })
    ));
    std::fs::remove_dir_all(root)
        .expect("7d83384c oversized_environment_example_is_rejected invariant must hold");
}
