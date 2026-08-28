#[frontend_contract::domain_types::route_error(AdminHtmlUserPasswordError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn user_password(
    auth: crate::AdminAuthReq,
    crate::AxumAdminForm(form): crate::AxumAdminForm<crate::UserPasswordForm>,
) -> axum::response::Response {
    crate::authenticated_action_impl::authenticated_action_impl(
        auth,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
        |auth| {
            crate::mutations_set_password::mutations_set_password(
                auth,
                crate::AxumAdminPath(crate::user_path_impl::user_path_impl(form.user_id)),
                crate::AxumAdminJson(
                    server_admin_contract::domain_types::AdminSetUserPasswordReq::new(
                        form.password,
                    ),
                ),
            )
        },
    )
    .await
}
