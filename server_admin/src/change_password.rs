#[frontend_contract_macros::route_error(AdminHtmlChangePasswordError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn change_password(
    auth: crate::admin_auth_req::AdminAuthReq,
    crate::axum_admin_form::AxumAdminForm(form): crate::axum_admin_form::AxumAdminForm<
        crate::change_password_form::ChangePasswordForm,
    >,
) -> axum::response::Response {
    match crate::form_auth_impl::form_auth_impl(auth) {
        Ok(auth) => {
            let request = server_admin_contract::admin_change_own_password_req::AdminChangeOwnPasswordReq::new(
                form.current_password,
                form.new_password,
            );
            match crate::account_change_own_password::account_change_own_password(
                auth,
                crate::axum_admin_json::AxumAdminJson(request),
            )
            .await
            {
                Ok(_response) => crate::success_redirect_impl::success_redirect_impl(
                    server_admin_contract::admin_frontend_path::AdminFrontendPath::Profile,
                ),
                Err(error) => axum::response::IntoResponse::into_response(error),
            }
        }
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}
