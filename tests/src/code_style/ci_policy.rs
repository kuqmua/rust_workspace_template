fn workflow() -> super::types::SourceText {
    super::types::SourceText::try_from(
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect("c02ae58b")
                .join(str_constants::code_style::CI_WORKFLOW_PATH),
        )
        .expect("da504e54"),
    )
    .expect("fd9f7861")
}
#[test]
#[allow(
    clippy::needless_for_each,
    reason = "repository source policy requires iterator methods instead of for loops"
)]
fn continuous_integration_contains_required_security_and_quality_commands() {
    let workflow = workflow();
    [
        str_constants::expr::S_1601,
        str_constants::expr::S_1680,
        str_constants::expr::S_1072,
        str_constants::expr::S_1071,
        str_constants::expr::S_0953,
        str_constants::expr::S_1070,
    ]
    .into_iter()
    .for_each(|required| assert!(workflow.as_ref().contains(required), "missing `{required}`"));
}
#[test]
fn workflow_jobs_have_timeouts_and_marketplace_actions_use_commit_shas() {
    let workflow = workflow();
    let workflow_jobs = workflow
        .as_ref()
        .split_once(str_constants::expr::S_1442)
        .map(|(_prefix, jobs)| jobs)
        .expect("ed8bc4d0");
    let mut inside_job = false;
    let mut current_job_has_timeout = false;
    workflow_jobs.lines().for_each(|line| {
        if line.starts_with(str_constants::expr::S_0002)
            && line.ends_with(':')
            && !line.starts_with(str_constants::expr::S_0000)
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
        if inside_job && line.trim_start().starts_with(str_constants::expr::S_1816) {
            current_job_has_timeout = true;
        }
        if let Some(action) = line.trim().strip_prefix(str_constants::expr::S_0047) {
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
