#[test]
fn runtime_code_does_not_use_expect_unwrap_or_panic() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::C71F2A8D),
        super::types::SourceTextRef::from(constants_str::RUNTIME_CODE_CONTAINS_FORBIDDEN_EXPECT_UNWRAP_PANIC_CALLS_USE_RESULT_WITH_A),
        |path, ast, ers| {
            if !super::is_runtime_policy_source_path(super::types::PathRef::from(path)).get() {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::runtime_analysis::RuntimePanicExpectUnwrapVisitor {
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
        super::types::StaticStr::from(constants_str::E3F8A1C5),
        super::types::SourceTextRef::from(constants_str::RUNTIME_CODE_CONTAINS_MUTEX_USE_IT_ONLY_FOR_JUSTIFIED_INTERIOR_MUTABILITY),
        |path, ast, ers| {
            if !super::is_runtime_policy_source_path(super::types::PathRef::from(path)).get() {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::runtime_analysis::RuntimeMutexVisitor {
                    found_count: super::types::AnalyzerCount::default(),
                },
            );
            super::push_repeated_file_error(
                super::types::DiagnosticMsgsMutRef::from(&mut *ers),
                super::types::PathRef::from(path),
                super::types::SourceTextRef::from(constants_str::MUTEX_TYPE_USAGE),
                visitor.found_count,
            );
        },
    );
}
#[test]
fn runtime_arc_usage_is_limited_to_cross_thread_state() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::F9C2D4A8),
        super::types::SourceTextRef::from(
            constants_str::RUNTIME_ARC_USAGE_MUST_BE_LIMITED_TO_EXPLICIT_CROSS_THREAD_SHARED_STATE,
        ),
        |path, ast, ers| {
            if !super::is_runtime_policy_source_path(super::types::PathRef::from(path)).get() {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::runtime_analysis::RuntimeArcVisitor {
                    allow_arc_value_usage: super::types::AnalyzerBool::from(
                        constants_str::CODE_STYLE_RUNTIME_ARC_OWNER_SUFFIXES
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
        .expect(
            "85acd272 runtime_test_crate_detection_uses_exact_package_names invariant must hold",
        );
    let test_crate = "[package]\nname = \"location_test\""
        .parse::<toml::Table>()
        .expect(
            "50b60550 runtime_test_crate_detection_uses_exact_package_names invariant must hold",
        );
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
        !super::is_runtime_policy_source_path(super::types::PathRef::from(std::path::Path::new(
            "../macros_helpers/src/domain_types/test_hlp.rs"
        )))
        .get(),
        "2e8a5d90"
    );
    assert!(
        super::is_runtime_policy_source_path(super::types::PathRef::from(std::path::Path::new(
            "../server/src/test_hlp.rs"
        )))
        .get(),
        "76c1f4b3"
    );
}
#[test]
fn environment_initializer_is_in_runtime_policy_scope() {
    assert!(
        super::is_runtime_policy_source_path(super::types::PathRef::from(std::path::Path::new(
            "../initialize_environment_files/src/domain_types.rs"
        )))
        .get(),
        "86c8a1dd"
    );
}
#[test]
fn async_functions_do_not_make_blocking_executor_calls() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::A8E1C6F3),
        super::types::SourceTextRef::from(
            constants_str::ASYNC_FUNCTIONS_CONTAIN_BLOCKING_EXECUTOR_CALLS,
        ),
        |path, ast, ers| {
            if !super::is_runtime_policy_source_path(super::types::PathRef::from(path)).get() {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::runtime_analysis::AsyncBlockingCallVisitor {
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
fn async_blocking_policy_rejects_sync_filesystem_network_and_executor_calls() {
    let ast = syn::parse_file(
        r#"
async fn blocked() {
    std::fs::read("input");
    std::net::TcpStream::connect("127.0.0.1:1");
    futures::executor::block_on(async {});
}
struct Service;
impl Service {
    async fn blocked_method() {
        std::fs::metadata("input");
    }
}
trait BlockedTrait {
    async fn blocked_default() {
        std::fs::canonicalize("input");
    }
}
fn nested_async() {
    let _future = async {
        std::fs::write("output", []);
    };
    let _closure = async || std::fs::read_to_string("input");
}
fn synchronous_is_allowed() {
    std::fs::read("input");
}
"#,
    )
    .expect("57a4f701 synchronous_is_allowed invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::runtime_analysis::AsyncBlockingCallVisitor {
            async_fn_depth: super::types::AnalyzerCount::default(),
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.ers.len(), 7usize);
}
#[test]
fn unit_tests_do_not_create_external_service_clients() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::D1F5B9C7),
        super::types::SourceTextRef::from(constants_str::UNIT_TESTS_CONTAIN_EXTERNAL_SERVICE_CLIENTS_USE_DETERMINISTIC_LOCAL_FAKES_INSTEAD),
        |path, ast, ers| {
            if super::is_test_source_path(super::types::PathRef::from(path)).get() {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::runtime_analysis::UnitTestExternalServiceVisitor {
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

#[test]
fn external_service_policy_rejects_http_database_and_socket_clients() {
    let ast = syn::parse_file(
        "#[test]
         fn external_clients() {
             reqwest::Client::builder();
             reqwest::get(\"https://example.invalid\");
             sqlx::postgres::PgPoolOptions::new().connect(\"postgres://example.invalid\");
             sqlx::PgPool::connect(\"postgres://example.invalid\");
             std::net::TcpStream::connect(\"127.0.0.1:1\");
         }",
    )
    .expect("62a4c3a8 external_clients invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::runtime_analysis::UnitTestExternalServiceVisitor {
            test_depth: super::types::AnalyzerCount::default(),
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.ers.len(), 5usize, "e165d841");
}

#[test]
fn external_service_policy_requires_a_reason_for_ignored_integration_tests() {
    let ast = syn::parse_file(
        "#[test]
         #[ignore]
         fn ignored_without_reason() {
             reqwest::get(\"https://example.invalid\");
         }
         #[test]
         #[ignore = \"requires an explicitly provisioned emulator\"]
         fn ignored_with_reason() {
             reqwest::get(\"https://example.invalid\");
         }",
    )
    .expect("fa48e32b ignored_with_reason invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::runtime_analysis::UnitTestExternalServiceVisitor {
            test_depth: super::types::AnalyzerCount::default(),
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.ers.len(), constants_usize::ONE, "31fd7ca0");
}
