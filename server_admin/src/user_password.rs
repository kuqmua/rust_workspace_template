#[frontend_contract::domain_types::route_error(AdminHtmlUserPasswordError)]
pub(super) async fn user_password(
    auth: super::super::super::super::AdminAuthReq,
    super::super::super::super::AxumAdminForm(form): super::super::super::super::AxumAdminForm<
        super::super::super::forms::UserPasswordForm,
    >,
) -> axum::response::Response {
    super::super::super::authenticated_action_impl::authenticated_action_impl(
        auth,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
        |auth| {
            super::super::super::super::users::mutations_set_password::mutations_set_password(
                auth,
                super::super::super::super::AxumAdminPath(
                    super::super::super::user_path_impl::user_path_impl(form.user_id),
                ),
                super::super::super::super::AxumAdminJson(
                    server_admin_contract::domain_types::AdminSetUserPasswordReq::new(
                        form.password,
                    ),
                ),
            )
        },
    )
    .await
}
