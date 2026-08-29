#[frontend_contract_macros::route_error(AdminHtmlUserBanError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn user_ban(
    auth: crate::admin_auth_req::AdminAuthReq,
    crate::axum_admin_form::AxumAdminForm(form): crate::axum_admin_form::AxumAdminForm<
        crate::user_ban_form::UserBanForm,
    >,
) -> axum::response::Response {
    crate::authenticated_action_impl::authenticated_action_impl(
        auth,
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Users,
        |auth| {
            crate::mutations_set_ban::mutations_set_ban(
                auth,
                crate::axum_admin_path::AxumAdminPath(crate::user_path_impl::user_path_impl(
                    form.user_id,
                )),
                crate::axum_admin_json::AxumAdminJson(
                    server_admin_contract::admin_set_user_ban_req::AdminSetUserBanReq::new(
                        form.is_banned,
                    ),
                ),
            )
        },
    )
    .await
}
