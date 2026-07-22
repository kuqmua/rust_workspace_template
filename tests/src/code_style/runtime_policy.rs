#[test]
fn runtime_code_does_not_use_expect_unwrap_or_panic() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::C71F2A8D),
        super::types::SourceTextRef::from(str_constants::RUNTIME_CODE_CONTAINS_FORBIDDEN_EXPECT_UNWRAP_PANIC_CALLS_USE_RESULT_WITH_A),
        |path, ast, ers| {
            if !super::is_runtime_policy_source_path(super::types::StdPathRef::from(path)).get() {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::RuntimePanicExpectUnwrapVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn runtime_code_does_not_use_mutex() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::E3F8A1C5),
        super::types::SourceTextRef::from(str_constants::RUNTIME_CODE_CONTAINS_MUTEX_USE_IT_ONLY_FOR_JUSTIFIED_INTERIOR_MUTABILITY),
        |path, ast, ers| {
            if !super::is_runtime_policy_source_path(super::types::StdPathRef::from(path)).get() {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::RuntimeMutexVisitor {
                    found_count: super::types::AnalyzerCount::default(),
                },
            );
            super::push_repeated_file_error(
                super::types::DiagnosticMsgsMutRef::from(&mut *ers),
                super::types::StdPathRef::from(path),
                super::types::SourceTextRef::from(str_constants::MUTEX_TYPE_USAGE),
                visitor.found_count,
            );
        },
    );
}
#[test]
fn runtime_arc_usage_is_limited_to_cross_thread_state() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::F9C2D4A8),
        super::types::SourceTextRef::from(
            str_constants::RUNTIME_ARC_USAGE_MUST_BE_LIMITED_TO_EXPLICIT_CROSS_THREAD_SHARED_STATE,
        ),
        |path, ast, ers| {
            if !super::is_runtime_policy_source_path(super::types::StdPathRef::from(path)).get() {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::RuntimeArcVisitor {
                    allow_arc_value_usage: super::types::AnalyzerBool::from(
                        str_constants::CODE_STYLE_RUNTIME_ARC_OWNER_SUFFIXES
                            .iter()
                            .any(|suffix| path.ends_with(suffix)),
                    ),
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn runtime_test_crate_detection_uses_exact_package_names() {
    let production = "[package]\nname = \"contest_service\""
        .parse::<toml::Table>()
        .expect("85acd272");
    let test_crate = "[package]\nname = \"location_test\""
        .parse::<toml::Table>()
        .expect("50b60550");
    assert!(
        !super::is_test_crate(super::types::TomlTableRef::from(&production)).get(),
        "3db51a9b"
    );
    assert!(
        super::is_test_crate(super::types::TomlTableRef::from(&test_crate)).get(),
        "6a5afda4"
    );
}
#[test]
fn runtime_test_helper_exclusion_is_file_exact() {
    assert!(
        !super::is_runtime_policy_source_path(super::types::StdPathRef::from(
            std::path::Path::new("../macros_helpers/src/test_hlp.rs")
        ))
        .get(),
        "2e8a5d90"
    );
    assert!(
        super::is_runtime_policy_source_path(super::types::StdPathRef::from(std::path::Path::new(
            "../server/src/test_hlp.rs"
        )))
        .get(),
        "76c1f4b3"
    );
}
#[test]
fn environment_initializer_is_in_runtime_policy_scope() {
    assert!(
        super::is_runtime_policy_source_path(super::types::StdPathRef::from(std::path::Path::new(
            "../initialize_environment_files/src/main.rs"
        )))
        .get(),
        "86c8a1dd"
    );
}
#[test]
fn async_functions_do_not_make_blocking_executor_calls() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::A8E1C6F3),
        super::types::SourceTextRef::from(
            str_constants::ASYNC_FUNCTIONS_CONTAIN_BLOCKING_EXECUTOR_CALLS,
        ),
        |path, ast, ers| {
            if !super::is_runtime_policy_source_path(super::types::StdPathRef::from(path)).get() {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::AsyncBlockingCallVisitor {
                    async_fn_depth: super::types::AnalyzerCount::default(),
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn unit_tests_do_not_create_external_service_clients() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(str_constants::D1F5B9C7),
        super::types::SourceTextRef::from(str_constants::UNIT_TESTS_CONTAIN_EXTERNAL_SERVICE_CLIENTS_USE_DETERMINISTIC_LOCAL_FAKES_INSTEAD),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::UnitTestExternalServiceVisitor {
                    test_depth: super::types::AnalyzerCount::default(),
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
