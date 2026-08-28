#[frontend_contract::route_error(AdminHtmlCreateUserError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn create_user(
    auth: crate::AdminAuthReq,
    crate::AxumAdminForm(form): crate::AxumAdminForm<crate::CreateUserForm>,
) -> axum::response::Response {
    let Ok(auth) = crate::form_auth_impl::form_auth_impl(auth) else {
        return axum::response::IntoResponse::into_response(crate::AdminError::Csrf);
    };
    let request = server_admin_contract::domain_types::AdminCreateUserReq::new(
        form.display_name,
        form.login,
        form.password,
    );
    crate::action_result_impl::action_result_impl(
        crate::user_mutations_create::user_mutations_create(auth, crate::AxumAdminJson(request))
            .await,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
    )
}
