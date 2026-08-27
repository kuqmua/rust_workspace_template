#[frontend_contract::domain_types::route_error(AdminHtmlChangePasswordError)]
pub(super) async fn change_password(
    auth: super::super::super::super::AdminAuthReq,
    super::super::super::super::AxumAdminForm(form): super::super::super::super::AxumAdminForm<
        super::super::super::forms::ChangePasswordForm,
    >,
) -> axum::response::Response {
    match super::super::super::form_auth_impl::form_auth_impl(auth) {
        Ok(auth) => {
            let request = server_admin_contract::domain_types::AdminChangeOwnPasswordReq::new(
                form.current_password,
                form.new_password,
            );
            match super::super::super::super::account_change_own_password::account_change_own_password(
                auth,
                super::super::super::super::AxumAdminJson(request),
            )
            .await
            {
                Ok(_response) => super::super::super::success_redirect_impl::success_redirect_impl(
                    server_admin_contract::domain_types::AdminFrontendPath::Profile,
                ),
                Err(error) => axum::response::IntoResponse::into_response(error),
            }
        }
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}
