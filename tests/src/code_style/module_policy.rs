const PRODUCTION_MODULE_MAX_LINES: usize = 2_500usize;
const INLINE_TEST_SEPARATION_MIN_LINES: usize = 1_024usize;
fn large_module_exceptions() -> [&'static str; 3] {
    [
        constants_str::VALUE_7FE2AF02,
        constants_str::VALUE_D405F3E1,
        constants_str::VALUE_B278317D,
    ]
}

fn is_test_source(path: &std::path::Path) -> bool {
    super::is_test_source_path(super::types::PathRef::from(path)).get()
        || path
            .components()
            .any(|component| component.as_os_str() == constants_str::VALUE_D0549AF3)
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
                    && !large_module_exceptions()
                        .iter()
                        .any(|exception| file.path().as_ref().ends_with(exception)))
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
            .filter(|file| {
                file.ast().as_ref().items.iter().any(|item| {
                    let syn::Item::Mod(module) = item else {
                        return false;
                    };
                    module.ident == constants_str::TESTS_ALT
                        && module.content.is_some()
                        && module.attrs.iter().any(|attribute| {
                            super::attr_is_test_only_cfg(super::types::SynAttributeRef::from(
                                attribute,
                            ))
                            .get()
                        })
                })
            })
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
                    .ends_with(constants_str::VALUE_8E41EC63)
            })
            .expect("2a6298c3 notification service domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split(constants_str::VALUE_3BA26FB4)
            .next()
            .expect("3ae48239 split always returns the production source prefix");
        [
            constants_str::VALUE_18A392BE,
            constants_str::VALUE_99D94433,
            constants_str::VALUE_96E8A555,
            constants_str::VALUE_5015C549,
            constants_str::VALUE_9B877603,
            constants_str::VALUE_8B0F112C,
            constants_str::VALUE_3A4EDC2D,
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
                    .ends_with(constants_str::VALUE_E5F20F68)
            })
            .expect("1dac054f environment initializer domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split(constants_str::VALUE_3BA26FB4)
            .next()
            .expect("fb55c47b split always returns the production source prefix");
        [
            constants_str::VALUE_59CAD555,
            constants_str::VALUE_B9D99DED,
            constants_str::VALUE_3349907E,
            constants_str::VALUE_7CF02D0B,
            constants_str::VALUE_13B1C208,
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
                    .ends_with(constants_str::VALUE_2C978AB0)
            })
            .expect("f49a25d6 administrator bootstrap domain types source must exist")
            .content()
            .as_ref();
        [
            constants_str::VALUE_E7118A3C,
            constants_str::VALUE_3349907E,
            constants_str::VALUE_FF3A4973,
            constants_str::VALUE_EB9EA192,
            constants_str::VALUE_9B877603,
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
                    .ends_with(constants_str::VALUE_30296F9B)
            })
            .expect("1ef73397 common route domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split(constants_str::VALUE_3BA26FB4)
            .next()
            .expect("90fc214f split always returns the production source prefix");
        [
            constants_str::VALUE_99D94433,
            constants_str::VALUE_2E84067B,
            constants_str::VALUE_C5EAB055,
            constants_str::VALUE_0E48D7B1,
            constants_str::VALUE_1812E35F,
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
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::VALUE_47325207)
            })
            .expect("2a49fec1 server domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split(constants_str::VALUE_3BA26FB4)
            .next()
            .expect("82d0ffa2 split always returns the production source prefix");
        [
            constants_str::VALUE_58D8E00E,
            constants_str::VALUE_4BB60066,
            constants_str::VALUE_5BE799DC,
            constants_str::VALUE_062AEA27,
            constants_str::VALUE_69B22E2A,
            constants_str::VALUE_9B877603,
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
                path.contains(constants_str::VALUE_5A1A4545)
                    || path.contains(constants_str::VALUE_43EF539D)
                    || path.ends_with(constants_str::VALUE_9CAC1060)
                    || path.ends_with(constants_str::VALUE_AAA5BED8)
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
                    .ends_with(constants_str::VALUE_1A456B0D)
            })
            .expect("3119b009 workspace scaffold domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split(constants_str::VALUE_3BA26FB4)
            .next()
            .expect("1e5e6186 split always returns the production source prefix");
        [
            constants_str::VALUE_59CAD555,
            constants_str::VALUE_E72B634A,
            constants_str::VALUE_731FDA74,
            constants_str::VALUE_522C24E5,
            constants_str::VALUE_C36F32EE,
            constants_str::VALUE_BCDC0F38,
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
                    .ends_with(constants_str::VALUE_712F68AD)
            })
            .expect("a081579c file storage domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split(constants_str::VALUE_3BA26FB4)
            .next()
            .expect("622c12de split always returns the production source prefix");
        [
            constants_str::TOKIO_PATH_FS_PATH,
            constants_str::VALUE_BAA7CB12,
            constants_str::VALUE_303C9B02,
            constants_str::VALUE_D30B72A0,
            constants_str::VALUE_B863F79E,
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
                    .ends_with(constants_str::VALUE_F45EC0EE)
            })
            .expect("8b3cb235 workspace test runner domain types source must exist")
            .content()
            .as_ref();
        let source = source_with_tests
            .split(constants_str::VALUE_3BA26FB4)
            .next()
            .expect("4c2a6281 split always returns the production source prefix");
        [
            constants_str::VALUE_32E64619,
            constants_str::VALUE_B9D99DED,
            constants_str::VALUE_7C10C158,
            constants_str::VALUE_E63A5758,
            constants_str::VALUE_9B877603,
            constants_str::VALUE_CC4BBDCE,
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
