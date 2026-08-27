use super::super::{GitCommitIdRef, ProjectGitInfo};

#[must_use]
pub fn project_git_info() -> ProjectGitInfo<'static> {
    ProjectGitInfo::from(GitCommitIdRef::from(
        constants_str::GIT_INFO_PROJECT_GIT_COMMIT_ID,
    ))
}
