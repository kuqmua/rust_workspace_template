fn workflow() -> super::types::SourceText {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("c02ae58b workflow invariant must hold")
            .join(constants_str::CODE_STYLE_CI_WORKFLOW_PATH),
    )
    .expect("da504e54 workflow invariant must hold");
    active_workflow_source(super::types::SourceTextRef::from(source.as_str()))
}
fn active_workflow_source(source: super::types::SourceTextRef<'_>) -> super::types::SourceText {
    super::types::SourceText::try_from(
        source
            .as_ref()
            .lines()
            .map(|line| active_yaml_line(super::types::SourceTextRef::from(line)).get())
            .collect::<Vec<&str>>()
            .join(constants_str::NEWLINE),
    )
    .expect("fd9f7861 active_workflow_source invariant must hold")
}
#[allow(
    clippy::single_call_fn,
    reason = "keeps YAML quote state isolated and fixture-testable"
)]
fn active_yaml_line(line: super::types::SourceTextRef<'_>) -> super::types::SourceTextRef<'_> {
    let comment_start = line
        .as_ref()
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
    super::types::SourceTextRef::from(comment_start.map_or_else(
        || line.get(),
        |index| {
            line.get()
                .get(..index)
                .expect("1a9e2f84 active_yaml_line invariant must hold")
        },
    ))
}
#[test]
#[allow(
    clippy::needless_for_each,
    reason = "repository source policy requires iterator methods instead of for loops"
)]
fn continuous_integration_contains_required_security_and_quality_commands() {
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
fn continuous_integration_runs_specialized_test_families() {
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
fn continuous_integration_uses_the_pinned_workspace_toolchain() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("01d2b547 continuous_integration_uses_the_pinned_workspace_toolchain invariant must hold");
    let toolchain_source = std::fs::read_to_string(
        repository_root.join(constants_str::VALUE_2B1BDE2C),
    )
    .expect(
        "f6db9220 continuous_integration_uses_the_pinned_workspace_toolchain invariant must hold",
    );
    let parsed_toolchain = toolchain_source.parse::<toml::Table>().expect(
        "874dc8b2 continuous_integration_uses_the_pinned_workspace_toolchain invariant must hold",
    );
    let toolchain = parsed_toolchain
        .get(constants_str::VALUE_0DB3DE82)
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get(constants_str::VALUE_69E36568))
        .and_then(toml::Value::as_str)
        .expect("a43da13b continuous_integration_uses_the_pinned_workspace_toolchain invariant must hold");
    let workflow = workflow();
    assert!(
        !workflow.as_ref().contains(toolchain),
        "the workflow must consume rust-toolchain.toml instead of repeating its channel"
    );
    assert!(
        !workflow.as_ref().contains("dtolnay/rust-toolchain"),
        "Rust jobs must use the repository-owned toolchain setup action"
    );
    assert!(
        workflow
            .as_ref()
            .contains("uses: ./.github/actions/setup-rust")
    );
    let setup_action = std::fs::read_to_string(
        repository_root.join(constants_str::VALUE_D8346474),
    )
    .expect(
        "830b79a6 continuous_integration_uses_the_pinned_workspace_toolchain invariant must hold",
    );
    assert!(setup_action.contains("rustc --version"));
    let services = std::fs::read_to_string(repository_root.join(constants_str::VALUE_C1590960))
        .expect("6b2f8d41 continuous_integration_uses_the_pinned_workspace_toolchain invariant must hold")
        .parse::<toml::Table>()
        .expect("1a7c5e93 continuous_integration_uses_the_pinned_workspace_toolchain invariant must hold");
    services
        .get(constants_str::SERVICE)
        .and_then(toml::Value::as_array)
        .expect("9d4e2b60 continuous_integration_uses_the_pinned_workspace_toolchain invariant must hold")
        .iter()
        .for_each(|service| {
            let dockerfile = service
                .as_table()
                .and_then(|table| table.get(constants_str::VALUE_254DB0FB))
                .and_then(toml::Value::as_str)
                .expect("3c8a1f72 continuous_integration_uses_the_pinned_workspace_toolchain invariant must hold");
            let source =
                std::fs::read_to_string(repository_root.join(dockerfile)).expect("5e9b4d16 continuous_integration_uses_the_pinned_workspace_toolchain invariant must hold");
            assert!(source.contains("rust-toolchain.toml"));
            assert!(!source.contains(toolchain));
        });
}
#[test]
fn workflow_jobs_have_timeouts_and_marketplace_actions_use_commit_shas() {
    let workflow = workflow();
    let workflow_jobs = workflow
        .as_ref()
        .split_once(constants_str::JOBS_NEWLINE)
        .map(|(_prefix, jobs)| jobs)
        .expect("ed8bc4d0 workflow_jobs_have_timeouts_and_marketplace_actions_use_commit_shas invariant must hold");
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
fn workflow_policy_ignores_commented_commands_and_actions() {
    let source = active_workflow_source(super::types::SourceTextRef::from(
        constants_str::VALUE_0356E8E3,
    ));
    assert!(!source.as_ref().contains(constants_str::CARGO_MACHETE));
    assert!(!source.as_ref().contains(constants_str::TIMEOUT_MINUTES));
    assert!(!source.as_ref().contains("actions/checkout"));
    assert!(source.as_ref().contains("quality # gate"));
    assert!(source.as_ref().contains("printf #active"));
}
