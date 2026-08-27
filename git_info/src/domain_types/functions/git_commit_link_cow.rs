use super::super::{
    BASE_GIT_COMMIT_LINK_LEN, GIT_INFO_STRING_MAX_LEN, GitCommitIdRef, GitCommitLinkCow,
    GitCommitLinkOutputRefMut, GitInfoStringTryFromStringError, git_commit_link_capacity,
    is_project_commit, project_git_commit_link_ref, write_git_commit_link,
};

#[must_use]
pub fn git_commit_link_cow<'commit_lt, CommitIdTy>(commit_id: CommitIdTy) -> GitCommitLinkCow
where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    let commit_id_ref = commit_id.into();
    if commit_id_ref.0.len() > GIT_INFO_STRING_MAX_LEN.saturating_sub(BASE_GIT_COMMIT_LINK_LEN) {
        return GitCommitLinkCow::try_from(std::borrow::Cow::Owned(
            GitInfoStringTryFromStringError::TooLong {
                len: BASE_GIT_COMMIT_LINK_LEN.saturating_add(commit_id_ref.0.len()),
                max: GIT_INFO_STRING_MAX_LEN,
            }
            .to_string(),
        ))
        .unwrap_or_else(GitCommitLinkCow::from);
    }
    if is_project_commit(commit_id_ref).0 {
        return GitCommitLinkCow::try_from(std::borrow::Cow::Borrowed(
            project_git_commit_link_ref().0,
        ))
        .unwrap_or_else(GitCommitLinkCow::from);
    }
    let cap = git_commit_link_capacity(commit_id_ref);
    let mut output = String::with_capacity(cap.0);
    let mut output_ref = GitCommitLinkOutputRefMut::from(&mut output);
    write_git_commit_link(&mut output_ref, commit_id_ref);
    GitCommitLinkCow::try_from(std::borrow::Cow::Owned(output))
        .unwrap_or_else(GitCommitLinkCow::from)
}
