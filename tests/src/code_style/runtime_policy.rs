#[test]
fn runtime_code_does_not_use_expect_unwrap_or_panic() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("c71f2a8d"),
        super::types::SourceTextRef::from(
            "runtime code contains forbidden expect/unwrap/panic calls; use Result with a \
             thiserror-like error enum instead:",
        ),
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
        super::types::StaticStr("e3f8a1c5"),
        super::types::SourceTextRef::from(
            "runtime code contains Mutex; use it only for justified interior mutability:",
        ),
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
                super::types::SourceTextRef::from("Mutex type usage"),
                visitor.found_count,
            );
        },
    );
}
#[test]
fn runtime_arc_usage_is_limited_to_cross_thread_state() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr("f9c2d4a8"),
        super::types::SourceTextRef::from(
            "runtime Arc usage must be limited to explicit cross-thread shared state:",
        ),
        |path, ast, ers| {
            if !super::is_runtime_policy_source_path(super::types::StdPathRef::from(path)).get() {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::RuntimeArcVisitor {
                    allow_arc_value_usage: super::types::AnalyzerBool::from(
                        path.ends_with("server/src/main.rs")
                            || path.ends_with("server_admin/src/password.rs")
                            || path.ends_with("server_runtime/src/bounded_read.rs"),
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
        super::types::StaticStr("a8e1c6f3"),
        super::types::SourceTextRef::from("async functions contain blocking executor calls:"),
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
        super::types::StaticStr("d1f5b9c7"),
        super::types::SourceTextRef::from(
            "unit tests contain external-service clients; use deterministic local fakes instead:",
        ),
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
