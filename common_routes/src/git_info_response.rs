#[frontend_contract::domain_types::route_openapi(tag = "service")]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(
    clippy::single_call_fn,
    reason = "route registry owns this Axum handler"
)]
pub(super) async fn git_info_response(
    app_state: crate::domain_types::ArcCommonRoutesAppState,
) -> crate::domain_types::JsonRes<crate::domain_types::GitInfo> {
    crate::domain_types::make_commit_json_response(
        app_state.get(),
        crate::domain_types::make_git_info_payload,
    )
}
