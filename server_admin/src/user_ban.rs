#[frontend_contract::domain_types::route_error(AdminHtmlUserBanError)]
pub(crate) async fn user_ban(
    auth: crate::AdminAuthReq,
    crate::AxumAdminForm(form): crate::AxumAdminForm<crate::UserBanForm>,
) -> axum::response::Response {
    crate::authenticated_action_impl::authenticated_action_impl(
        auth,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
        |auth| {
            crate::mutations_set_ban::mutations_set_ban(
                auth,
                crate::AxumAdminPath(crate::user_path_impl::user_path_impl(form.user_id)),
                crate::AxumAdminJson(
                    server_admin_contract::domain_types::AdminSetUserBanReq::new(form.is_banned),
                ),
            )
        },
    )
    .await
}
