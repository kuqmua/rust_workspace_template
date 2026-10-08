fn fixture(init_path_ref: crate::init_path_ref::InitPathRef<'_>) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(init_path_ref.get());
    (if root.exists() {
        std::fs::remove_dir_all(&root)
    } else {
        Ok(())
    })
    .and_then(|()| std::fs::create_dir_all(root.join(constants_str::SERVICE)))
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
fn test_environment_write_failure_preserves_typed_io_source() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(constants_str::CARGO_TOML)
        .join(constants_str::X);
    let expected = std::fs::File::open(&path).err();
    assert!(expected.is_some());
    let result = crate::write_content::write_content(
        crate::init_path_ref::InitPathRef::from(path.as_path()),
        crate::env_content_ref::EnvContentRef::from(constants_str::X),
    );
    assert!(result.is_err_and(|error| {
        matches!(
            &error,
            crate::initialize_error::InitializeError::WriteEnvironment { .. }
        ) && std::error::Error::source(&error).is_some_and(|source| {
            source.is::<crate::init_io_error::InitIoError>()
                && expected
                    .as_ref()
                    .is_some_and(|expected_error| source.to_string() == expected_error.to_string())
        })
    }));
}

#[test]
fn test_dry_run_apply_and_repeat_are_safe_and_idempotent() {
    let root = fixture(crate::init_path_ref::InitPathRef::from(
        std::path::Path::new(stringify!(
            test_dry_run_apply_and_repeat_are_safe_and_idempotent
        )),
    ));
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
    let dry_update = crate::initialize::initialize(
        crate::workspace_root_path_ref::WorkspaceRootPathRef::from(root.as_path()),
        crate::run_mode::RunMode::DryRun,
    );
    assert!(
        dry_update.is_ok_and(|entries| entries.as_ref().first().is_some_and(|entry| {
            entry.status() == crate::initialization_status::InitializationStatus::WouldUpdate
                && entry.member().as_ref() == constants_str::SERVICE
                && entry.keys().as_ref().len().get() == 2usize
        }))
    );
    assert!(
        std::fs::read_to_string(root.join(constants_str::SERVICE_ENV))
            .is_ok_and(|content| content == constants_str::SECRET_CUSTOM_NEWLINE)
    );
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
    assert!(
        std::fs::remove_file(root.join(constants_str::SERVICE_ENV_EXAMPLE)).is_ok_and(|()| true)
    );
    assert!(
        crate::initialize::initialize(
            crate::workspace_root_path_ref::WorkspaceRootPathRef::from(root.as_path()),
            crate::run_mode::RunMode::DryRun,
        )
        .is_ok_and(|entries| entries.as_ref().is_empty())
    );
    assert!(
        std::fs::read_to_string(root.join(constants_str::SERVICE_ENV))
            .is_ok_and(|content| content == updated_content)
    );
    std::fs::remove_dir_all(root).expect(constants_str::DIAGNOSTIC_BD9180CA);
}

#[test]
fn test_initialization_manifest_failures_preserve_distinct_error_stages() {
    let root = fixture(crate::init_path_ref::InitPathRef::from(
        std::path::Path::new(stringify!(
            test_initialization_manifest_failures_preserve_distinct_error_stages
        )),
    ));
    let manifest_path = root.join(constants_str::CARGO_TOML);
    assert!(std::fs::remove_file(&manifest_path).is_ok_and(|()| true));
    let result = crate::initialize::initialize(
        crate::workspace_root_path_ref::WorkspaceRootPathRef::from(root.as_path()),
        crate::run_mode::RunMode::DryRun,
    );
    assert!(result.is_err_and(|error| matches!(
        &error,
        crate::initialize_error::InitializeError::ReadManifest { .. }
    ) && std::error::Error::source(&error).is_some_and(
        <dyn std::error::Error>::is::<server_runtime_http::bounded_read_error::BoundedReadError>
    )));
    let invalid_manifest = '['.to_string();
    let expected_diagnostic = toml::from_str::<toml::Value>(&invalid_manifest)
        .err()
        .map(|error| error.to_string());
    assert!(expected_diagnostic.is_some());
    assert!(std::fs::write(&manifest_path, &invalid_manifest).is_ok_and(|()| true));
    let parse_result = crate::initialize::initialize(
        crate::workspace_root_path_ref::WorkspaceRootPathRef::from(root.as_path()),
        crate::run_mode::RunMode::DryRun,
    );
    assert!(parse_result.is_err_and(
        |error| matches!(error, crate::initialize_error::InitializeError::ManifestParse { source }
        if Some(source.to_string()) == expected_diagnostic)
    ));
    [
        constants_str::EMPTY.to_owned(),
        constants_str::WORKSPACE_NEWLINE_MEMBERS_SERVICE_NEWLINE
            .replace(constants_str::MEMBERS, constants_str::X),
    ]
    .into_iter()
    .fold((), |(), manifest| {
        assert!(std::fs::write(&manifest_path, manifest).is_ok_and(|()| true));
        assert!(matches!(
            crate::initialize::initialize(
                crate::workspace_root_path_ref::WorkspaceRootPathRef::from(root.as_path()),
                crate::run_mode::RunMode::DryRun,
            ),
            Err(crate::initialize_error::InitializeError::MembersMissing)
        ));
    });
    assert!(std::fs::remove_dir_all(root).is_ok_and(|()| true));
}
#[test]
fn test_escaping_member_is_rejected() {
    let root = fixture(crate::init_path_ref::InitPathRef::from(
        std::path::Path::new(stringify!(test_escaping_member_is_rejected)),
    ));
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
    let root = fixture(crate::init_path_ref::InitPathRef::from(
        std::path::Path::new(stringify!(test_oversized_environment_example_is_rejected)),
    ));
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
#[test]
fn test_non_string_workspace_member_is_rejected() {
    let value = toml::Value::Integer(1);
    let parsed = crate::workspace_member::WorkspaceMember::try_from(
        crate::toml_member_value::TomlMemberValue::from(&value),
    );
    assert!(matches!(
        parsed,
        Err(crate::initialize_error::InitializeError::InvalidMemberType)
    ));
}

#[test]
fn test_environment_key_preserves_byte_limits_and_owned_storage() {
    assert!(
        [
            constants_str::X.to_owned(),
            constants_str::X.repeat(1_024usize),
            '\u{00e9}'.to_string().repeat(512usize),
        ]
        .into_iter()
        .all(|input| {
            let length = input.len();
            let pointer = input.as_ptr();
            crate::env_key::EnvKey::try_from(input).is_ok_and(|env_key| {
                env_key.as_ref().len() == length && env_key.as_ref().as_ptr() == pointer
            })
        })
    );
    assert!(
        [
            String::new(),
            constants_str::X.repeat(1_025usize),
            '\u{00e9}'.to_string().repeat(513usize),
        ]
        .into_iter()
        .all(|input| matches!(
            crate::env_key::EnvKey::try_from(input),
            Err(crate::init_string_error::InitStringError::Invalid)
        ))
    );
}

#[test]
fn test_environment_key_parser_filters_lines_preserves_order_and_propagates_errors() {
    let parser_input = [
        ' ', 'x', ' ', '=', '1', '=', '2', '\n', '#', 'x', '=', '3', '\n', 'a', '\n', 'x', '=',
        '4', '\n', '\t', 'y', '=', '5', '\n', ' ', '\n',
    ]
    .into_iter()
    .collect::<String>();
    assert!(
        crate::environment_keys::environment_keys(crate::env_content_ref::EnvContentRef::from(
            parser_input.as_str()
        ))
        .is_ok_and(|env_keys| {
            let keys = env_keys.as_ref().as_slice();
            keys.len() == 3usize
                && keys
                    .iter()
                    .zip(['x', 'x', 'y'])
                    .all(|(env_key, expected)| env_key.as_ref().chars().eq([expected]))
        })
    );
    assert!(
        crate::environment_keys::environment_keys(crate::env_content_ref::EnvContentRef::from(
            constants_str::EMPTY
        ))
        .is_ok_and(|env_keys| env_keys.as_ref().is_empty())
    );
    let mut oversized_key = constants_str::X.repeat(1_025usize);
    oversized_key.push('=');
    assert!(
        [
            [' ', '=', 'x'].into_iter().collect::<String>(),
            oversized_key,
        ]
        .into_iter()
        .all(|input| matches!(
            crate::environment_keys::environment_keys(crate::env_content_ref::EnvContentRef::from(
                input.as_str()
            )),
            Err(crate::init_string_error::InitStringError::Invalid)
        ))
    );
}

#[test]
fn test_workspace_member_text_and_toml_path_boundaries() {
    assert!(
        [
            constants_str::SERVICE.to_owned(),
            constants_str::X.repeat(4_096usize),
            '\u{00e9}'.to_string().repeat(2_048usize),
        ]
        .into_iter()
        .all(|input| {
            let length = input.len();
            let pointer = input.as_ptr();
            crate::workspace_member::WorkspaceMember::try_from(input).is_ok_and(
                |workspace_member| {
                    workspace_member.as_ref().len() == length
                        && workspace_member.as_ref().as_ptr() == pointer
                },
            )
        })
    );
    assert!(
        [
            String::new(),
            constants_str::X.repeat(4_097usize),
            '\u{00e9}'.to_string().repeat(2_049usize),
        ]
        .into_iter()
        .all(|input| {
            let value = toml::Value::String(input);
            matches!(
                crate::workspace_member::WorkspaceMember::try_from(
                    crate::toml_member_value::TomlMemberValue::from(&value)
                ),
                Err(crate::initialize_error::InitializeError::String(
                    crate::init_string_error::InitStringError::Invalid
                ))
            )
        })
    );
    assert!(
        [
            constants_str::SLASH.to_owned(),
            constants_str::TEST_PATH_TRAVERSAL.to_owned(),
            '.'.to_string(),
        ]
        .into_iter()
        .all(|input| {
            let value = toml::Value::String(input);
            matches!(
                crate::workspace_member::WorkspaceMember::try_from(
                    crate::toml_member_value::TomlMemberValue::from(&value)
                ),
                Err(crate::initialize_error::InitializeError::InvalidMember { .. })
            )
        })
    );
    let nested = [
        constants_str::SERVICE,
        constants_str::SLASH,
        constants_str::X,
    ]
    .concat();
    let value = toml::Value::String(nested);
    assert!(
        crate::workspace_member::WorkspaceMember::try_from(
            crate::toml_member_value::TomlMemberValue::from(&value)
        )
        .is_ok_and(|workspace_member| {
            value
                .as_str()
                .is_some_and(|text| workspace_member.as_ref() == text)
        })
    );
}

#[test]
fn test_environment_merge_preserves_existing_values_and_normalizes_missing_newline() {
    let root = fixture(crate::init_path_ref::InitPathRef::from(
        std::path::Path::new(stringify!(
            test_environment_merge_preserves_existing_values_and_normalizes_missing_newline
        )),
    ));
    let public_line = constants_str::PUBLIC_VALUE_NEWLINE_SECRET_CHANGE_ME_NEWLINE
        .lines()
        .take(1usize)
        .flat_map(|line| line.chars().chain(std::iter::once('\n')))
        .collect::<String>();
    let expected_updated = [constants_str::SECRET_CUSTOM_NEWLINE, public_line.as_str()].concat();
    [
        (
            constants_str::EMPTY,
            constants_str::PUBLIC_VALUE_NEWLINE_SECRET_CHANGE_ME_NEWLINE,
        ),
        (
            constants_str::SECRET_CUSTOM_NEWLINE.trim_end_matches('\n'),
            expected_updated.as_str(),
        ),
        (
            constants_str::SECRET_CUSTOM_NEWLINE,
            expected_updated.as_str(),
        ),
    ]
    .into_iter()
    .fold((), |(), (current, expected)| {
        assert!(
            std::fs::write(root.join(constants_str::SERVICE_ENV), current).is_ok_and(|()| true)
        );
        assert!(
            crate::initialize::initialize(
                crate::workspace_root_path_ref::WorkspaceRootPathRef::from(root.as_path()),
                crate::run_mode::RunMode::Apply,
            )
            .is_ok_and(|entries| entries.as_ref().first().is_some_and(|entry| {
                entry.status() == crate::initialization_status::InitializationStatus::Updated
                    && entry.member().as_ref() == constants_str::SERVICE
                    && entry.keys().as_ref().len().get() == 2usize
            }))
        );
        assert!(
            std::fs::read_to_string(root.join(constants_str::SERVICE_ENV))
                .is_ok_and(|content| content == expected)
        );
        assert!(
            crate::initialize::initialize(
                crate::workspace_root_path_ref::WorkspaceRootPathRef::from(root.as_path()),
                crate::run_mode::RunMode::Apply,
            )
            .is_ok_and(|entries| entries.as_ref().first().is_some_and(|entry| {
                entry.status()
                    == crate::initialization_status::InitializationStatus::SkippedExisting
            }))
        );
        assert!(
            std::fs::read_to_string(root.join(constants_str::SERVICE_ENV))
                .is_ok_and(|content| content == expected)
        );
    });
    assert!(std::fs::remove_dir_all(root).is_ok_and(|()| true));
}

#[test]
fn test_environment_initializer_rejects_invalid_utf8_in_example_and_existing_content() {
    let root = fixture(crate::init_path_ref::InitPathRef::from(
        std::path::Path::new(stringify!(
            test_environment_initializer_rejects_invalid_utf8_in_example_and_existing_content
        )),
    ));
    [
        constants_str::SERVICE_ENV_EXAMPLE,
        constants_str::SERVICE_ENV,
    ]
    .into_iter()
    .fold((), |(), relative_path| {
        assert!(
            std::fs::write(
                root.join(constants_str::SERVICE_ENV_EXAMPLE),
                constants_str::PUBLIC_VALUE_NEWLINE_SECRET_CHANGE_ME_NEWLINE
            )
            .is_ok_and(|()| true)
        );
        assert!(
            std::fs::write(
                root.join(constants_str::SERVICE_ENV),
                constants_str::SECRET_CUSTOM_NEWLINE
            )
            .is_ok_and(|()| true)
        );
        assert!(std::fs::write(root.join(relative_path), [u8::MAX]).is_ok_and(|()| true));
        assert!(matches!(
            crate::initialize::initialize(
                crate::workspace_root_path_ref::WorkspaceRootPathRef::from(root.as_path()),
                crate::run_mode::RunMode::Apply,
            ),
            Err(crate::initialize_error::InitializeError::ReadExample {
                source: server_runtime_http::bounded_read_error::BoundedReadError::Utf8 { .. }
            })
        ));
        assert!(std::fs::read(root.join(relative_path)).is_ok_and(|content| content == [u8::MAX]));
    });
    assert!(std::fs::remove_dir_all(root).is_ok_and(|()| true));
}
