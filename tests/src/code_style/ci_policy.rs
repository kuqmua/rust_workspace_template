fn workflow() -> super::types::SourceText {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("c02ae58b")
            .join(str_constants::CODE_STYLE_CI_WORKFLOW_PATH),
    )
    .expect("da504e54");
    active_workflow_source(super::types::SourceTextRef::from(source.as_str()))
}
fn active_workflow_source(source: super::types::SourceTextRef<'_>) -> super::types::SourceText {
    super::types::SourceText::try_from(
        source
            .as_ref()
            .lines()
            .map(|line| active_yaml_line(super::types::SourceTextRef::from(line)).get())
            .collect::<Vec<&str>>()
            .join("\n"),
    )
    .expect("fd9f7861")
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
        |index| line.get().get(..index).expect("1a9e2f84"),
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
        str_constants::PERMISSIONS_NEWLINE_CONTENTS_READ,
        str_constants::RHYSD_ACTIONLINT,
        str_constants::CARGO_MACHETE,
        str_constants::CARGO_LLVM_COV_WORKSPACE_ALL_FEATURES_SUMMARY_ONLY,
        str_constants::AQUASECURITY_TRIVY_ACTION,
        str_constants::CARGO_PLUS_NIGHTLY_UDEPS_WORKSPACE_ALL_TARGETS_ALL_FEATURES_LOCKED,
    ]
    .into_iter()
    .for_each(|required| assert!(workflow.as_ref().contains(required), "missing `{required}`"));
}
#[test]
fn continuous_integration_uses_the_pinned_workspace_toolchain() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("01d2b547");
    let toolchain_source =
        std::fs::read_to_string(repository_root.join("rust-toolchain.toml")).expect("f6db9220");
    let parsed_toolchain = toolchain_source.parse::<toml::Table>().expect("874dc8b2");
    let toolchain = parsed_toolchain
        .get("toolchain")
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get("channel"))
        .and_then(toml::Value::as_str)
        .expect("a43da13b");
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
    let setup_action =
        std::fs::read_to_string(repository_root.join(".github/actions/setup-rust/action.yml"))
            .expect("830b79a6");
    assert!(setup_action.contains("rustc --version"));
    let services = std::fs::read_to_string(repository_root.join("deploy/services.toml"))
        .expect("6b2f8d41")
        .parse::<toml::Table>()
        .expect("1a7c5e93");
    services
        .get("service")
        .and_then(toml::Value::as_array)
        .expect("9d4e2b60")
        .iter()
        .for_each(|service| {
            let dockerfile = service
                .as_table()
                .and_then(|table| table.get("dockerfile"))
                .and_then(toml::Value::as_str)
                .expect("3c8a1f72");
            let source =
                std::fs::read_to_string(repository_root.join(dockerfile)).expect("5e9b4d16");
            assert!(source.contains("rust-toolchain.toml"));
            assert!(!source.contains(toolchain));
        });
}
#[test]
fn workflow_jobs_have_timeouts_and_marketplace_actions_use_commit_shas() {
    let workflow = workflow();
    let workflow_jobs = workflow
        .as_ref()
        .split_once(str_constants::JOBS_NEWLINE)
        .map(|(_prefix, jobs)| jobs)
        .expect("ed8bc4d0");
    let mut inside_job = false;
    let mut current_job_has_timeout = false;
    workflow_jobs.lines().for_each(|line| {
        if line.starts_with(str_constants::TWO_SPACES)
            && line.ends_with(':')
            && !line.starts_with(str_constants::FOUR_SPACES)
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
            && line.starts_with(str_constants::FOUR_SPACES)
            && !line
                .strip_prefix(str_constants::FOUR_SPACES)
                .is_some_and(|remainder| remainder.starts_with(str_constants::TWO_SPACES))
            && line
                .trim_start()
                .starts_with(str_constants::TIMEOUT_MINUTES)
        {
            current_job_has_timeout = true;
        }
        if let Some(action) = line.trim().strip_prefix(str_constants::USES) {
            if action.starts_with("./") {
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
        "# cargo machete\n# uses: actions/checkout@0123456789012345678901234567890123456789\nname: \"quality # gate\"\nrun: 'printf #active'\njobs:\n  check:\n    # timeout-minutes: 10\n    runs-on: ubuntu-latest\n",
    ));
    assert!(!source.as_ref().contains(str_constants::CARGO_MACHETE));
    assert!(!source.as_ref().contains(str_constants::TIMEOUT_MINUTES));
    assert!(!source.as_ref().contains("actions/checkout"));
    assert!(source.as_ref().contains("quality # gate"));
    assert!(source.as_ref().contains("printf #active"));
}
