#[test]
fn test_runtime_code_does_not_use_expect_unwrap_or_panic() {
    crate::code_style::assert_rs_ast_ers_empty_with_ctx(
        crate::types::StaticStr::from(constants_str::C71F2A8D),
        crate::types::SourceTextRef::from(constants_str::RUNTIME_CODE_CONTAINS_FORBIDDEN_EXPECT_UNWRAP_PANIC_CALLS_USE_RESULT_WITH_A),
        |path, ast, ers| {
            if !crate::code_style::is_runtime_policy_source_path(crate::types::PathRef::from(path)).get() {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::runtime_analysis::RuntimePanicExpectUnwrapVisitor::new(crate::types::DiagnosticMsgs::default()),
            );
            ers.extend(
                visitor
                    .get_ers().clone().into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_runtime_code_does_not_use_mutex() {
    crate::code_style::assert_rs_ast_ers_empty_with_ctx(
        crate::types::StaticStr::from(constants_str::E3F8A1C5),
        crate::types::SourceTextRef::from(constants_str::RUNTIME_CODE_CONTAINS_MUTEX_USE_IT_ONLY_FOR_JUSTIFIED_INTERIOR_MUTABILITY),
        |path, ast, ers| {
            if !crate::code_style::is_runtime_policy_source_path(crate::types::PathRef::from(path)).get() {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::runtime_analysis::RuntimeMutexVisitor::new(crate::types::AnalyzerCount::default()),
            );
            crate::code_style::push_repeated_file_error(
                crate::types::DiagnosticMsgsMutRef::from(&mut *ers),
                crate::types::PathRef::from(path),
                crate::types::SourceTextRef::from(constants_str::MUTEX_TYPE_USAGE),
                *visitor.get_found_count(),
            );
        },
    );
}
#[test]
fn test_runtime_arc_usage_is_limited_to_cross_thread_state() {
    let defines_explicit_shared_arc_wrapper = |ast: &syn::File| {
        ast.items.iter().any(|item| {
            let syn::Item::Struct(item_struct) = item else {
                return false;
            };
            let name = item_struct.ident.to_string();
            let explicitly_shared =
                name.contains(constants_str::ARC) || name.contains(constants_str::SHARED);
            explicitly_shared
                && item_struct.fields.iter().any(|field| {
                    let syn::Type::Path(path) = &field.ty else {
                        return false;
                    };
                    path.path.segments.iter().any(|segment| {
                        let field_type_name = segment.ident.to_string();
                        field_type_name.contains(constants_str::ARC)
                            || field_type_name.contains(constants_str::SHARED)
                    })
                })
        })
    };
    crate::code_style::assert_rs_ast_ers_empty_with_ctx(
        crate::types::StaticStr::from(constants_str::F9C2D4A8),
        crate::types::SourceTextRef::from(
            constants_str::RUNTIME_ARC_USAGE_MUST_BE_LIMITED_TO_EXPLICIT_CROSS_THREAD_SHARED_STATE,
        ),
        |path, ast, ers| {
            if !crate::code_style::is_runtime_policy_source_path(crate::types::PathRef::from(path))
                .get()
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::runtime_analysis::RuntimeArcVisitor::new(
                    crate::types::DiagnosticMsgs::default(),
                    crate::types::AnalyzerBool::from(defines_explicit_shared_arc_wrapper(ast)),
                ),
            );
            ers.extend(
                visitor
                    .get_ers()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_runtime_test_crate_detection_uses_test_name_segments() {
    let production = constants_str::VALUE_D0480B8C
        .parse::<toml::Table>()
        .expect(constants_str::DIAGNOSTIC_85ACD272);
    let test_crate = constants_str::VALUE_D6812408
        .parse::<toml::Table>()
        .expect(constants_str::DIAGNOSTIC_50B60550);
    assert!(
        !crate::code_style::is_test_crate(crate::types::TomlTableRef::from(&production)).get(),
        "3db51a9b"
    );
    assert!(
        crate::code_style::is_test_crate(crate::types::TomlTableRef::from(&test_crate)).get(),
        "6a5afda4"
    );
}
#[test]
fn test_runtime_test_module_exclusion_uses_test_filename() {
    assert!(
        !crate::code_style::is_runtime_policy_source_path(crate::types::PathRef::from(
            std::path::Path::new("../server_admin_frontend/src/crud_tests.rs")
        ))
        .get(),
        "2e8a5d90"
    );
    assert!(
        crate::code_style::is_runtime_policy_source_path(crate::types::PathRef::from(
            std::path::Path::new("../server/src/test_helper.rs")
        ))
        .get(),
        "76c1f4b3"
    );
}
#[test]
fn test_environment_initializer_is_in_runtime_policy_scope() {
    assert!(
        crate::code_style::is_runtime_policy_source_path(crate::types::PathRef::from(
            std::path::Path::new("../init_env_files/src/initialize.rs")
        ))
        .get(),
        "86c8a1dd"
    );
}
#[test]
fn test_async_functions_do_not_make_blocking_executor_calls() {
    crate::code_style::assert_rs_ast_ers_empty_with_ctx(
        crate::types::StaticStr::from(constants_str::A8E1C6F3),
        crate::types::SourceTextRef::from(
            constants_str::ASYNC_FUNCTIONS_CONTAIN_BLOCKING_EXECUTOR_CALLS,
        ),
        |path, ast, ers| {
            if !crate::code_style::is_runtime_policy_source_path(crate::types::PathRef::from(path))
                .get()
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::runtime_analysis::AsyncBlockingCallVisitor::new(
                    crate::types::AnalyzerCount::default(),
                    crate::types::DiagnosticMsgs::default(),
                ),
            );
            ers.extend(
                visitor
                    .get_ers()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_async_blocking_policy_rejects_sync_filesystem_network_and_executor_calls() {
    let ast =
        syn::parse_file(constants_str::VALUE_9AC9CBBD).expect(constants_str::DIAGNOSTIC_57A4F701);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::runtime_analysis::AsyncBlockingCallVisitor::new(
            crate::types::AnalyzerCount::default(),
            crate::types::DiagnosticMsgs::default(),
        ),
    );
    assert_eq!(visitor.get_ers().len(), 7usize);
}
#[test]
fn test_unit_tests_do_not_create_external_service_clients() {
    crate::code_style::assert_rs_ast_ers_empty_with_ctx(
        crate::types::StaticStr::from(constants_str::D1F5B9C7),
        crate::types::SourceTextRef::from(constants_str::UNIT_TESTS_CONTAIN_EXTERNAL_SERVICE_CLIENTS_USE_DETERMINISTIC_LOCAL_FAKES_INSTEAD),
        |path, ast, ers| {
            if crate::code_style::is_test_source_path(crate::types::PathRef::from(path)).get() {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::runtime_analysis::UnitTestExternalServiceVisitor::new(crate::types::DiagnosticMsgs::default(), crate::types::AnalyzerCount::default()),
            );
            ers.extend(
                visitor
                    .get_ers().clone().into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}

#[test]
fn test_external_service_policy_rejects_http_database_and_socket_clients() {
    let ast =
        syn::parse_file(constants_str::VALUE_0FE6CFEC).expect(constants_str::DIAGNOSTIC_62A4C3A8);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::runtime_analysis::UnitTestExternalServiceVisitor::new(
            crate::types::DiagnosticMsgs::default(),
            crate::types::AnalyzerCount::default(),
        ),
    );
    assert_eq!(visitor.get_ers().len(), 5usize, "e165d841");
}

#[test]
fn test_external_service_policy_requires_a_reason_for_ignored_integration_tests() {
    let ast =
        syn::parse_file(constants_str::VALUE_7BBB4BBC).expect(constants_str::DIAGNOSTIC_FA48E32B);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::runtime_analysis::UnitTestExternalServiceVisitor::new(
            crate::types::DiagnosticMsgs::default(),
            crate::types::AnalyzerCount::default(),
        ),
    );
    assert_eq!(visitor.get_ers().len(), constants_usize::ONE, "31fd7ca0");
}
