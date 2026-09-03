#[proc_macro_frontend_contract::route_error(AdminHtmlCreateUserError)]
#[allow(
    clippy::single_call_fn,
    reason = "create user remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn create_user(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_form: crate::axum_admin_form::AxumAdminForm<crate::create_user_form::CreateUserForm>,
) -> axum::response::Response {
    let Ok(auth) = crate::form_auth_impl::form_auth_impl(admin_auth_request) else {
        return axum::response::IntoResponse::into_response(crate::admin_error::AdminError::Csrf);
    };
    let request = server_admin_contract::admin_create_user_request::AdminCreateUserRequest::new(
        axum_admin_form.get_display_name().clone(),
        axum_admin_form.get_login().clone(),
        axum_admin_form.get_password().clone(),
    );
    crate::action_result_impl::action_result_impl(
        crate::user_mutations_create::user_mutations_create(
            auth,
            crate::axum_admin_json::AxumAdminJson::from(request),
        )
        .await,
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Users,
    )
}
