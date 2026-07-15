#[test]
fn runtime_code_does_not_use_expect_unwrap_or_panic() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr(str_constants::expr::S_1052),
        super::types::SourceTextRef::from(str_constants::expr::S_1694),
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
        super::types::StaticStr(str_constants::expr::S_1216),
        super::types::SourceTextRef::from(str_constants::expr::S_1693),
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
                super::types::SourceTextRef::from(str_constants::expr::S_0703),
                visitor.found_count,
            );
        },
    );
}
#[test]
fn runtime_arc_usage_is_limited_to_cross_thread_state() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr(str_constants::expr::S_1306),
        super::types::SourceTextRef::from(str_constants::expr::S_1692),
        |path, ast, ers| {
            if !super::is_runtime_policy_source_path(super::types::StdPathRef::from(path)).get() {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::RuntimeArcVisitor {
                    allow_arc_value_usage: super::types::AnalyzerBool::from(
                        path.ends_with(str_constants::expr::S_1721)
                            || path.ends_with(str_constants::expr::S_1723)
                            || path.ends_with(str_constants::expr::S_1729),
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
        super::types::StaticStr(str_constants::expr::S_0894),
        super::types::SourceTextRef::from(str_constants::expr::S_0968),
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
        super::types::StaticStr(str_constants::expr::S_1119),
        super::types::SourceTextRef::from(str_constants::expr::S_1852),
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
