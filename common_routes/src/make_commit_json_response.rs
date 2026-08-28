use super::{JsonRes, make_json_response};

#[allow(
    clippy::single_call_fn,
    reason = "response composition remains directly unit tested"
)]
pub(crate) fn make_commit_json_response<S, T>(
    commit_src: &S,
    map: impl FnOnce(git_info::domain_types::GitCommitLinkCow) -> T,
) -> JsonRes<T>
where
    S: ?Sized + git_info::domain_types::GitCommitLinkProvider,
{
    make_json_response(map(
        git_info::domain_types::GitCommitLinkProvider::build_git_commit_link_cow(commit_src),
    ))
}
