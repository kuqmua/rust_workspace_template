use super::super::ProjectGitCommitLinkRef;

#[must_use]
pub fn project_git_commit_link_ref() -> ProjectGitCommitLinkRef {
    ProjectGitCommitLinkRef::from(constants_str::GIT_INFO_PROJECT_GIT_COMMIT_LINK)
}
