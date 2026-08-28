#[frontend_contract::route_openapi(tag = "service")]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(
    clippy::single_call_fn,
    reason = "route registry owns this Axum handler"
)]
pub(super) async fn git_info_response(
    app_state: crate::ArcCommonRoutesAppState,
) -> crate::JsonRes<crate::GitInfo> {
    crate::make_json_response(crate::GitInfo {
        commit: git_info::GitCommitLinkProvider::build_git_commit_link_cow(app_state.get()),
    })
}
