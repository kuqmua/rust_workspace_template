#[must_use]
pub fn project_git_info_value() -> crate::project_git_info::ProjectGitInfo<'static> {
    crate::project_git_info::ProjectGitInfo::from(crate::git_commit_id_ref::GitCommitIdRef::from(
        constants_str::GIT_INFO_PROJECT_GIT_COMMIT_ID,
    ))
}
