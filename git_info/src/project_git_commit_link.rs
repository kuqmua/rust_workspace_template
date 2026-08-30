#[must_use]
pub fn project_git_commit_link() -> crate::git_commit_link::GitCommitLink {
    crate::git_commit_link::GitCommitLink::try_from(
        <&str>::from(crate::project_git_commit_link_ref_value::project_git_commit_link_ref_value())
            .to_owned(),
    )
    .unwrap_or_else(crate::git_commit_link::GitCommitLink::from)
}
