#[proc_macro_frontend_contract::route_openapi(tag = "service")]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(
    clippy::single_call_fn,
    reason = "route registry owns this Axum handler"
)]
pub(super) async fn git_info_response(
    arc_common_routes_app_state: crate::arc_common_routes_app_state::ArcCommonRoutesAppState,
) -> crate::json_response::JsonResponse<crate::git_info::GitInfo> {
    crate::make_json_response::make_json_response(crate::git_info::GitInfo::from_commit(
        git_info::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link_cow(
            arc_common_routes_app_state.get(),
        ),
    ))
}
