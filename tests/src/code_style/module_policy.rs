const PRODUCTION_MODULE_MAX_LINES: usize = 2_500usize;
const INLINE_TEST_SEPARATION_MIN_LINES: usize = 1_024usize;
fn large_module_exceptions() -> [&'static str; 3] {
    [
        "pg_crud_pg_table_generate_src/src/domain_types/source.rs",
        "pg_crud_pg_types_generate_src/src/domain_types/source.rs",
        "constants_str/src/lib.rs",
    ]
}

fn is_test_source(path: &std::path::Path) -> bool {
    super::is_test_source_path(super::types::PathRef::from(path)).get()
        || path
            .components()
            .any(|component| component.as_os_str() == "test_fixtures")
}

#[allow(
    clippy::single_call_fn,
    reason = "the named AST predicate keeps module ownership detection separate from assertion flow"
)]
fn has_inline_test_module(ast: &syn::File) -> bool {
    ast.items.iter().any(|item| {
        let syn::Item::Mod(module) = item else {
            return false;
        };
        module.ident == "tests"
            && module.content.is_some()
            && module.attrs.iter().any(|attribute| {
                super::attr_is_test_only_cfg(super::types::SynAttributeRef::from(attribute)).get()
            })
    })
}

#[allow(
    clippy::single_call_fn,
    reason = "the named predicate centralizes the exact reviewed exception match"
)]
fn is_large_module_exception(path: &std::path::Path) -> bool {
    large_module_exceptions()
        .iter()
        .any(|exception| path.ends_with(exception))
}

#[test]
fn production_modules_have_bounded_responsibility() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|file| !is_test_source(file.path().as_ref()))
            .filter_map(|file| {
                let line_count = file.content().as_ref().lines().count();
                (line_count > PRODUCTION_MODULE_MAX_LINES
                    && !is_large_module_exception(file.path().as_ref()))
                .then(|| format!("{}: {line_count} lines", file.path().as_ref().display()))
            })
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "production modules above {PRODUCTION_MODULE_MAX_LINES} lines must be split by responsibility:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
fn large_production_modules_keep_tests_in_separate_files() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|file| !is_test_source(file.path().as_ref()))
            .filter(|file| {
                file.content().as_ref().lines().count() > INLINE_TEST_SEPARATION_MIN_LINES
            })
            .filter(|file| has_inline_test_module(file.ast().as_ref()))
            .map(|file| file.path().as_ref().display().to_string())
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "large production modules must keep tests in separate files:\n{}",
            violations.join("\n")
        );
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn large_module_exceptions_are_exact_and_still_needed() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        large_module_exceptions().iter().for_each(|exception| {
            let matching_file = snapshot
                .rs_files()
                .iter()
                .find(|file| file.path().as_ref().ends_with(exception));
            let Some(file) = matching_file else {
                panic!("2d8f4a61 missing large-module exception target: {exception}");
            };
            assert!(
                file.content().as_ref().lines().count() > PRODUCTION_MODULE_MAX_LINES,
                "9a71c5e3 stale large-module exception: {exception}"
            );
        });
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn notification_service_domain_types_exclude_application_and_adapter_workflows() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let source_with_tests = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with("notification_service/src/domain_types.rs")
            })
            .expect("2a6298c3 notification service domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split("#[cfg(test)]")
            .next()
            .expect("3ae48239 split always returns the production source prefix");
        [
            "#[tokio::main]",
            "sqlx::query(",
            "TcpListener::bind(",
            "serve_with_graceful_shutdown(",
            "fn run_main(",
            "fn migrate_notification(",
            "fn create_notification(",
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(
                !source.contains(forbidden),
                "8e620507 notification service domain_types contains adapter or application workflow `{forbidden}`"
            );
        });
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn environment_initializer_domain_types_exclude_entrypoint_orchestration() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let source_with_tests = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with("initialize_environment_files/src/domain_types.rs")
            })
            .expect("1dac054f environment initializer domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split("#[cfg(test)]")
            .next()
            .expect("fb55c47b split always returns the production source prefix");
        [
            "std::env::args()",
            "std::fs::write(",
            "read_bounded_file(",
            "tracing::info!(",
            "fn run()",
        ]
            .iter()
            .for_each(|forbidden| {
                assert!(
                    !source.contains(forbidden),
                    "5d5bcd83 environment initializer domain_types contains entrypoint workflow `{forbidden}`"
                );
            });
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn admin_bootstrap_domain_types_exclude_application_workflows() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let source = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with("admin_bootstrap/src/domain_types.rs")
            })
            .expect("f49a25d6 administrator bootstrap domain types source must exist")
            .content()
            .as_ref();
        [
            "std::env::args_os()",
            "read_bounded_file(",
            "PgPoolOptions::new()",
            "tokio::runtime::Builder",
            "fn run_main(",
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(
                !source.contains(forbidden),
                "2f5b1520 administrator bootstrap domain_types contains application workflow `{forbidden}`"
            );
        });
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn common_route_domain_types_exclude_http_and_database_workflows() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let source_with_tests = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with("common_routes/src/domain_types.rs")
            })
            .expect("1ef73397 common route domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split("#[cfg(test)]")
            .next()
            .expect("90fc214f split always returns the production source prefix");
        [
            "sqlx::query(",
            "route_registry(",
            "async fn health_live(",
            "async fn health_ready(",
            "pub fn common_routes(",
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(
                !source.contains(forbidden),
                "6e3c94a8 common route domain_types contains HTTP or database workflow `{forbidden}`"
            );
        });
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn server_domain_types_exclude_application_and_adapter_workflows() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let source_with_tests = snapshot
            .rs_files()
            .iter()
            .find(|file| file.path().as_ref().ends_with("server/src/domain_types.rs"))
            .expect("2a49fec1 server domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split("#[cfg(test)]")
            .next()
            .expect("82d0ffa2 split always returns the production source prefix");
        [
            "sqlx::postgres::PgPoolOptions",
            "tokio::net::TcpListener::bind(",
            "axum::Router::new()",
            "async fn run_server(",
            "async fn migrate_server(",
            "fn run_main(",
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(
                !source.contains(forbidden),
                "f8d1eb2d server domain_types contains application or adapter workflow `{forbidden}`"
            );
        });
    });
}

#[test]
fn server_admin_domain_types_exclude_repository_workflows() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|file| {
                let path = file.path().as_ref().to_string_lossy();
                path.contains("server_admin/src/domain_types/auth")
                    || path.contains("server_admin/src/domain_types/repository")
                    || path.ends_with("server_admin/src/domain_types/migrations.rs")
                    || path.ends_with("server_admin/src/domain_types/cleanup.rs")
            })
            .map(|file| file.path().as_ref().display().to_string())
            .collect::<Vec<_>>();
        assert!(
            violations.is_empty(),
            "5aee4dc0 server admin domain_types contains application or persistence workflows: {violations:?}"
        );
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn workspace_scaffold_domain_types_exclude_entrypoint_and_template_filesystem_workflows() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let source_with_tests = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with("workspace_scaffold/src/domain_types.rs")
            })
            .expect("3119b009 workspace scaffold domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split("#[cfg(test)]")
            .next()
            .expect("1e5e6186 split always returns the production source prefix");
        [
            "std::env::args()",
            "fn run_ok(",
            "fn workspace_root(",
            "std::fs::read_dir(",
            "fn copy_template_tree(",
            "fn rename_identity(",
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(
                !source.contains(forbidden),
                "762cf3b3 workspace scaffold domain_types contains entrypoint or template filesystem workflow `{forbidden}`"
            );
        });
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn file_storage_domain_types_exclude_filesystem_workflows() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let source_with_tests = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with("file_storage/src/domain_types.rs")
            })
            .expect("a081579c file storage domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split("#[cfg(test)]")
            .next()
            .expect("622c12de split always returns the production source prefix");
        [
            "tokio::fs::",
            "AsyncWriteExt::",
            "fn stage_upload(",
            "fn commit_delete(",
            "fn cleanup_stale_staging(",
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(
                !source.contains(forbidden),
                "894ee8e8 file storage domain_types contains filesystem workflow `{forbidden}`"
            );
        });
    });
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the iterator form follows the workspace no-for-loop policy"
)]
fn workspace_test_runner_domain_types_exclude_application_and_adapter_workflows() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let source_with_tests = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with("workspace_test_runner/src/domain_types.rs")
            })
            .expect("8b3cb235 workspace test runner domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split("#[cfg(test)]")
            .next()
            .expect("4c2a6281 split always returns the production source prefix");
        [
            "std::process::exit(",
            "std::fs::write(",
            "ToolCommand::new(",
            "eprintln!(",
            "fn run_main(",
            "fn measure_memusage_command(",
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(
                !source.contains(forbidden),
                "f071d2cf workspace test runner domain_types contains application or adapter workflow `{forbidden}`"
            );
        });
    });
}
