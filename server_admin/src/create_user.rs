#[proc_macro_frontend_contract::route_error(AdminHtmlCreateUserError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn create_user(
    auth: crate::admin_auth_req::AdminAuthReq,
    form: crate::axum_admin_form::AxumAdminForm<crate::create_user_form::CreateUserForm>,
) -> axum::response::Response {
    let Ok(auth) = crate::form_auth_impl::form_auth_impl(auth) else {
        return axum::response::IntoResponse::into_response(crate::admin_error::AdminError::Csrf);
    };
    let request = server_admin_contract::admin_create_user_req::AdminCreateUserReq::new(
        form.get_display_name().clone(),
        form.get_login().clone(),
        form.get_password().clone(),
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
