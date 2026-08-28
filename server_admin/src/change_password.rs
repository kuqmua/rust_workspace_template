#[frontend_contract::route_error(AdminHtmlChangePasswordError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn change_password(
    auth: crate::AdminAuthReq,
    crate::AxumAdminForm(form): crate::AxumAdminForm<crate::ChangePasswordForm>,
) -> axum::response::Response {
    match crate::form_auth_impl::form_auth_impl(auth) {
        Ok(auth) => {
            let request = server_admin_contract::domain_types::AdminChangeOwnPasswordReq::new(
                form.current_password,
                form.new_password,
            );
            match crate::account_change_own_password::account_change_own_password(
                auth,
                crate::AxumAdminJson(request),
            )
            .await
            {
                Ok(_response) => crate::success_redirect_impl::success_redirect_impl(
                    server_admin_contract::domain_types::AdminFrontendPath::Profile,
                ),
                Err(error) => axum::response::IntoResponse::into_response(error),
            }
        }
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}
