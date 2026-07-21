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
                        path.ends_with(str_constants::SERVER_SRC_MAIN_RS)
                            || path.ends_with(str_constants::SERVER_ADMIN_SRC_PASSWORD_RS)
                            || path.ends_with(str_constants::SERVER_RUNTIME_SRC_BOUNDED_READ_RS)
                            || path.ends_with(str_constants::SERVER_RUNTIME_SRC_LIMITS_RS),
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
