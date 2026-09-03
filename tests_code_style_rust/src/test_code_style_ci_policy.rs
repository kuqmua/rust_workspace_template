fn workflow() -> crate::types::SourceText {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect(constants_str::DIAGNOSTIC_C02AE58B)
            .join(constants_str::CODE_STYLE_CI_WORKFLOW_PATH),
    )
    .expect(constants_str::DIAGNOSTIC_DA504E54);
    active_workflow_source(crate::types::SourceTextRef::from(source.as_str()))
}
fn active_workflow_source(
    source_text_ref: crate::types::SourceTextRef<'_>,
) -> crate::types::SourceText {
    crate::types::SourceText::try_from(
        source_text_ref
            .as_ref()
            .lines()
            .map(|line| {
                let comment_start = line
                    .char_indices()
                    .try_fold(
                        (false, false, false),
                        |(inside_single, inside_double, escaped), (index, character)| {
                            if character == '#' && !inside_single && !inside_double {
                                return Err(index);
                            }
                            let next_escaped = inside_double && character == '\\' && !escaped;
                            let next_single = if character == '\'' && !inside_double && !escaped {
                                !inside_single
                            } else {
                                inside_single
                            };
                            let next_double = if character == '"' && !inside_single && !escaped {
                                !inside_double
                            } else {
                                inside_double
                            };
                            Ok((next_single, next_double, next_escaped))
                        },
                    )
                    .err();
                comment_start.map_or_else(
                    || line,
                    |index| line.get(..index).expect(constants_str::DIAGNOSTIC_1A9E2F84),
                )
            })
            .collect::<Vec<&str>>()
            .join(constants_str::NEWLINE),
    )
    .expect(constants_str::DIAGNOSTIC_FD9F7861)
}
#[test]
#[allow(
    clippy::needless_for_each,
    reason = "repository source policy requires iterator methods instead of for loops"
)]
fn test_continuous_integration_contains_required_security_and_quality_commands() {
    let workflow = workflow();
    [
        constants_str::PERMISSIONS_NEWLINE_CONTENTS_READ,
        constants_str::RHYSD_ACTIONLINT,
        constants_str::CARGO_MACHETE,
        constants_str::CARGO_LLVM_COV_WORKSPACE_ALL_FEATURES_SUMMARY_ONLY,
        constants_str::AQUASECURITY_TRIVY_ACTION,
        constants_str::CARGO_PLUS_NIGHTLY_UDEPS_WORKSPACE_ALL_TARGETS_ALL_FEATURES_LOCKED,
    ]
    .into_iter()
    .for_each(|required| assert!(workflow.as_ref().contains(required), "missing `{required}`"));
}
#[test]
#[allow(
    clippy::needless_for_each,
    reason = "repository source policy requires iterator methods instead of for loops"
)]
fn test_continuous_integration_runs_specialized_test_families() {
    let workflow = workflow();
    [
        constants_str::CI_MIRI_COMPONENT,
        constants_str::CI_MIRI_TEST_CMD,
        constants_str::CI_DATABASE_TEST_CMD,
        constants_str::CI_BROWSER_TEST_CMD,
    ]
    .into_iter()
    .for_each(|required| {
        assert!(
            workflow.as_ref().contains(required),
            "specialized test family command is missing: `{required}`"
        );
    });
}
#[test]
fn test_workspace_test_runner_runs_code_style_once() {
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_STYLE_ARGS
            .contains(&constants_str::TESTS_CODE_STYLE_RUST)
    );
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_WORKSPACE_ARGS
            .windows(constants_usize::TWO)
            .any(|args| {
                args == [
                    constants_str::SHARED_VALUES_EXCLUDE,
                    constants_str::TESTS_CODE_STYLE_RUST,
                ]
            })
    );
}
#[test]
fn test_continuous_integration_uses_the_pinned_workspace_toolchain() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect(constants_str::DIAGNOSTIC_01D2B547);
    let toolchain_source =
        std::fs::read_to_string(repository_root.join(constants_str::VALUE_2B1BDE2C))
            .expect(constants_str::DIAGNOSTIC_F6DB9220);
    let parsed_toolchain = toolchain_source
        .parse::<toml::Table>()
        .expect(constants_str::DIAGNOSTIC_874DC8B2);
    let toolchain = parsed_toolchain
        .get(constants_str::VALUE_0DB3DE82)
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get(constants_str::VALUE_69E36568))
        .and_then(toml::Value::as_str)
        .expect(constants_str::DIAGNOSTIC_A43DA13B);
    let workflow = workflow();
    assert!(
        !workflow.as_ref().contains(toolchain),
        "the workflow must consume rust-toolchain.toml instead of repeating its channel"
    );
    assert!(
        !workflow.as_ref().contains(constants_str::VALUE_3563945F),
        "Rust jobs must use the repository-owned toolchain setup action"
    );
    assert!(workflow.as_ref().contains(constants_str::VALUE_394455C3));
    let setup_action = std::fs::read_to_string(repository_root.join(constants_str::VALUE_D8346474))
        .expect(constants_str::DIAGNOSTIC_830B79A6);
    assert!(setup_action.contains(constants_str::VALUE_0F49E23E));
    let services = std::fs::read_to_string(repository_root.join(constants_str::VALUE_C1590960))
        .expect(constants_str::DIAGNOSTIC_6B2F8D41)
        .parse::<toml::Table>()
        .expect(constants_str::DIAGNOSTIC_1A7C5E93);
    services
        .get(constants_str::SERVICE)
        .and_then(toml::Value::as_array)
        .expect(constants_str::DIAGNOSTIC_9D4E2B60)
        .iter()
        .for_each(|service| {
            let dockerfile = service
                .as_table()
                .and_then(|table| table.get(constants_str::VALUE_254DB0FB))
                .and_then(toml::Value::as_str)
                .expect(constants_str::DIAGNOSTIC_3C8A1F72);
            let source = std::fs::read_to_string(repository_root.join(dockerfile))
                .expect(constants_str::DIAGNOSTIC_5E9B4D16);
            assert!(source.contains(constants_str::VALUE_2B1BDE2C));
            assert!(!source.contains(toolchain));
        });
}
#[test]
fn test_workflow_jobs_have_timeouts_and_marketplace_actions_use_commit_shas() {
    let workflow = workflow();
    let workflow_jobs = workflow
        .as_ref()
        .split_once(constants_str::JOBS_NEWLINE)
        .map(|(_prefix, jobs)| jobs)
        .expect(constants_str::DIAGNOSTIC_ED8BC4D0);
    let mut inside_job = false;
    let mut current_job_has_timeout = false;
    workflow_jobs.lines().for_each(|line| {
        if line.starts_with(constants_str::TWO_SPACES)
            && line.ends_with(':')
            && !line.starts_with(constants_str::FOUR_SPACES)
        {
            if inside_job {
                assert!(
                    current_job_has_timeout,
                    "a workflow job lacks timeout-minutes"
                );
            }
            inside_job = true;
            current_job_has_timeout = false;
        }
        if inside_job
            && line.starts_with(constants_str::FOUR_SPACES)
            && !line
                .strip_prefix(constants_str::FOUR_SPACES)
                .is_some_and(|remainder| remainder.starts_with(constants_str::TWO_SPACES))
            && line
                .trim_start()
                .starts_with(constants_str::TIMEOUT_MINUTES)
        {
            current_job_has_timeout = true;
        }
        if let Some(action) = line.trim().strip_prefix(constants_str::USES) {
            if action.starts_with(constants_str::VALUE_C14CECEC) {
                return;
            }
            let revision = action.rsplit_once('@').map(|(_, revision)| revision);
            assert!(
                revision.is_some_and(|value| {
                    value.len() == 40usize && value.bytes().all(|byte| byte.is_ascii_hexdigit())
                }),
                "action `{action}` is not pinned to a full commit SHA"
            );
        }
    });
    assert!(
        !inside_job || current_job_has_timeout,
        "a workflow job lacks timeout-minutes"
    );
}
#[test]
fn test_workflow_policy_ignores_commented_commands_and_actions() {
    let source = active_workflow_source(crate::types::SourceTextRef::from(
        constants_str::VALUE_0356E8E3,
    ));
    assert!(!source.as_ref().contains(constants_str::CARGO_MACHETE));
    assert!(!source.as_ref().contains(constants_str::TIMEOUT_MINUTES));
    assert!(!source.as_ref().contains(constants_str::VALUE_728EABD6));
    assert!(source.as_ref().contains(constants_str::VALUE_3B4E324D));
    assert!(source.as_ref().contains(constants_str::VALUE_769E5F7B));
}
