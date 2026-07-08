#[test]
fn runtime_code_does_not_use_expect_unwrap_or_panic() {
    super::assert_rs_ast_ers_empty_with_ctx(
        "c71f2a8d",
        "runtime code contains forbidden expect/unwrap/panic calls; use Result with a \
             thiserror-like error enum instead:",
        |path, ast, ers| {
            if !super::is_runtime_policy_source_path(path) {
                return;
            }
            let visitor = super::visit_syn_file(
                ast,
                super::RuntimePanicExpectUnwrapVisitor { ers: Vec::new() },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|er| format!("{}: {er}", path.display())),
            );
        },
    );
}
#[test]
fn runtime_code_does_not_use_mutex() {
    super::assert_rs_ast_ers_empty_with_ctx(
        "e3f8a1c5",
        "runtime code contains Mutex; use it only for justified interior mutability:",
        |path, ast, ers| {
            if !super::is_runtime_policy_source_path(path) {
                return;
            }
            let visitor = super::visit_syn_file(ast, super::RuntimeMutexVisitor { found_count: 0 });
            super::push_repeated_file_er(ers, path, "Mutex type usage", visitor.found_count);
        },
    );
}
#[test]
fn runtime_arc_usage_is_limited_to_cross_thread_state() {
    super::assert_rs_ast_ers_empty_with_ctx(
        "f9c2d4a8",
        "runtime Arc usage must be limited to explicit cross-thread shared state:",
        |path, ast, ers| {
            if !super::is_runtime_policy_source_path(path) {
                return;
            }
            let visitor = super::visit_syn_file(
                ast,
                super::RuntimeArcVisitor {
                    allow_arc_value_usage: path.ends_with("server/src/main.rs"),
                    ers: Vec::new(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|er| format!("{}: {er}", path.display())),
            );
        },
    );
}
#[test]
fn async_functions_do_not_make_blocking_executor_calls() {
    super::assert_rs_ast_ers_empty_with_ctx(
        "a8e1c6f3",
        "async functions contain blocking executor calls:",
        |path, ast, ers| {
            if !super::is_runtime_policy_source_path(path) {
                return;
            }
            let visitor = super::visit_syn_file(
                ast,
                super::AsyncBlockingCallVisitor {
                    async_fn_depth: 0,
                    ers: Vec::new(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|er| format!("{}: {er}", path.display())),
            );
        },
    );
}
#[test]
fn unit_tests_do_not_create_external_service_clients() {
    super::assert_rs_ast_ers_empty_with_ctx(
        "d1f5b9c7",
        "unit tests contain external-service clients; use deterministic local fakes instead:",
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                ast,
                super::UnitTestExternalServiceVisitor {
                    test_depth: 0,
                    ers: Vec::new(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|er| format!("{}: {er}", path.display())),
            );
        },
    );
}
