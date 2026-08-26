fn assert_file_content(path: &std::path::Path, expected: &str) {
    let actual =
        std::fs::read_to_string(path).expect("371dbe92 assert_file_content invariant must hold");
    assert_eq!(actual, expected, "239c17b0: {}", path.display());
}

fn write(path: &std::path::Path, value: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("2f0ad03a write invariant must hold");
    }
    std::fs::write(path, value).expect("79af6dc8 write invariant must hold");
}

#[test]
fn validates_and_converts_project_names() {
    let valid = super::ProjectNameRef::from(constants_str::VALUE_F9EA74B8);
    super::naming_validate_project_name::naming_validate_project_name(valid)
        .expect("96de3a80 validates_and_converts_project_names invariant must hold");
    assert_eq!(
        super::naming_kebab_case::naming_kebab_case(valid).as_ref(),
        "order-platform"
    );
    assert_eq!(
        super::naming_title_case::naming_title_case(valid).as_ref(),
        "Order Platform"
    );
    assert_eq!(
        super::naming_upper_camel_case::naming_upper_camel_case(valid).as_ref(),
        "OrderPlatform"
    );
    assert!(
        super::naming_validate_project_name::naming_validate_project_name(super::ProjectNameRef(
            "Order-Platform"
        ))
        .is_err()
    );
}

#[test]
fn requires_https_repository_url() {
    super::naming_validate_repository_url::naming_validate_repository_url(
        super::RepositoryUrlRef::from(constants_str::VALUE_A680FDEF),
    )
    .expect("28c1e7a4 requires_https_repository_url invariant must hold");
    assert!(
        super::naming_validate_repository_url::naming_validate_repository_url(
            super::RepositoryUrlRef("http://example.com/team/order_platform")
        )
        .is_err()
    );
}

#[test]
fn deployment_projection_check_rejects_stale_generated_content() {
    let path = std::env::temp_dir().join(format!(
        "workspace-scaffold-generated-test-{}",
        std::process::id()
    ));
    let begin = constants_str::VALUE_0BAD8889;
    let end = constants_str::VALUE_79B72852;
    write(path.as_path(), constants_str::VALUE_0889759C);
    let check = super::synchronize_generated_file(
        super::ScaffoldPathRef::from(path.as_path()),
        super::ScaffoldTextRef::from(begin),
        super::ScaffoldTextRef::from(end),
        super::ScaffoldTextRef::from(constants_str::VALUE_48AA6CAE),
        super::ShouldWrite::from(false),
    );
    assert!(matches!(
        check,
        Err(super::ScaffoldError::GeneratedDeployment)
    ));
    super::synchronize_generated_file(
        super::ScaffoldPathRef::from(path.as_path()),
        super::ScaffoldTextRef::from(begin),
        super::ScaffoldTextRef::from(end),
        super::ScaffoldTextRef::from(constants_str::VALUE_48AA6CAE),
        super::ShouldWrite::from(true),
    )
    .expect(
        "5a7e3c91 deployment_projection_check_rejects_stale_generated_content invariant must hold",
    );
    super::synchronize_generated_file(
        super::ScaffoldPathRef::from(path.as_path()),
        super::ScaffoldTextRef::from(begin),
        super::ScaffoldTextRef::from(end),
        super::ScaffoldTextRef::from(constants_str::VALUE_48AA6CAE),
        super::ShouldWrite::from(false),
    )
    .expect(
        "d2f8b4a6 deployment_projection_check_rejects_stale_generated_content invariant must hold",
    );
    std::fs::remove_file(path).expect(
        "9c1e6a3f deployment_projection_check_rejects_stale_generated_content invariant must hold",
    );
}

#[test]
fn service_catalog_owns_ci_and_release_projection_values() {
    let entries = super::service_catalog_parse::service_catalog_parse(
        super::ScaffoldTextRef::from(constants_str::VALUE_D4291B4A),
    )
    .expect("4e8b2d7a service_catalog_owns_ci_and_release_projection_values invariant must hold");
    let entries_ref = super::ServiceCatalogEntriesRef::from(entries.0.as_slice());
    assert_eq!(
        super::service_catalog_render_ci_matrix::service_catalog_render_ci_matrix(entries_ref)
            .as_ref(),
        "          - name: application\n            dockerfile: Dockerfile\n"
    );
    assert_eq!(
        super::service_catalog_render_release_matrix::service_catalog_render_release_matrix(
            entries_ref
        )
        .as_ref(),
        "          - name: application\n            dockerfile: Dockerfile\n"
    );
}

#[test]
fn rejects_scaffold_text_over_size_limit() {
    let path = std::env::temp_dir().join(format!(
        "workspace-scaffold-oversize-test-{}",
        std::process::id()
    ));
    std::fs::write(
        path.as_path(),
        vec![b'x'; constants_usize::VALUE_16_777_216.saturating_add(constants_usize::ONE)],
    )
    .expect("d97e30ac rejects_scaffold_text_over_size_limit invariant must hold");
    let result = crate::adapters::template_fs_read_bounded_text::template_fs_read_bounded_text(
        super::ScaffoldPathRef::from(path.as_path()),
    );
    assert!(
        matches!(
            result,
            Err(super::ServerRuntimeBoundedReadError(
                server_runtime_http::domain_types::BoundedReadError::ExceedsMaximum { .. }
            ))
        ),
        "8f32bc16"
    );
    std::fs::remove_file(path)
        .expect("51cd7b2e rejects_scaffold_text_over_size_limit invariant must hold");
}

#[test]
fn service_scaffold_registers_all_artifacts() {
    let root = std::env::temp_dir().join(format!("workspace-scaffold-test-{}", std::process::id()));
    if root.exists() {
        std::fs::remove_dir_all(root.as_path())
            .expect("1449608d service_scaffold_registers_all_artifacts invariant must hold");
    }
    write(
        root.join(constants_str::CARGO_TOML).as_path(),
        constants_str::VALUE_9A836A5B,
    );
    write(
        root.join(constants_str::VALUE_8E41EC63).as_path(),
        constants_str::VALUE_45AD55F9,
    );
    write(
        root.join(constants_str::VALUE_F7C1AF06).as_path(),
        constants_str::VALUE_244072F2,
    );
    write(
        root.join(constants_str::VALUE_0A7A2313).as_path(),
        constants_str::VALUE_B3508161,
    );
    write(
        root.join(constants_str::VALUE_4F50C4FE).as_path(),
        constants_str::VALUE_A64251C2,
    );
    write(
        root.join(constants_str::VALUE_09101A6F).as_path(),
        constants_str::VALUE_04354311,
    );
    write(
        root.join(constants_str::VALUE_13A8EB94).as_path(),
        constants_str::VALUE_D0FC32F7,
    );
    write(
        root.join(constants_str::VALUE_C1590960).as_path(),
        constants_str::VALUE_D4E98611,
    );
    crate::domain_types::scaffold_service(
        super::ScaffoldPathRef::from(root.as_path()),
        super::ProjectNameRef::from(constants_str::VALUE_E896B9AF),
        super::ServicePort::from(8082u16),
    )
    .expect("4bff1d79 insert_sql invariant must hold");
    assert_file_content(
        root.join(constants_str::CARGO_TOML).as_path(),
        constants_str::VALUE_ADF1A200,
    );
    assert_file_content(
        root.join(constants_str::VALUE_7654C453).as_path(),
        constants_str::VALUE_2120BC93,
    );
    assert_file_content(
        root.join(constants_str::VALUE_D3EA3646).as_path(),
        constants_str::VALUE_77C620D8,
    );
    assert_file_content(
        root.join(constants_str::VALUE_0626DBBE).as_path(),
        constants_str::VALUE_6DC62C71,
    );
    assert_file_content(
        root.join(constants_str::VALUE_83CBEECD).as_path(),
        constants_str::VALUE_7602E17D,
    );
    assert_file_content(
        root.join(constants_str::VALUE_13A8EB94).as_path(),
        constants_str::VALUE_9A2A3063,
    );
    assert_file_content(
        root.join(constants_str::VALUE_7D4D7140).as_path(),
        constants_str::VALUE_499A1FF6,
    );
    assert_file_content(
        root.join(constants_str::VALUE_C1590960).as_path(),
        constants_str::VALUE_142D5AD3,
    );
    std::fs::remove_dir_all(root).expect("6f608418 insert_sql invariant must hold");
}
