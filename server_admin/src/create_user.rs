#[frontend_contract::domain_types::route_error(AdminHtmlCreateUserError)]
pub(super) async fn create_user(
    auth: super::super::super::super::AdminAuthReq,
    super::super::super::super::AxumAdminForm(form): super::super::super::super::AxumAdminForm<
        super::super::super::forms::CreateUserForm,
    >,
) -> axum::response::Response {
    let Ok(auth) = super::super::super::form_auth_impl::form_auth_impl(auth) else {
        return axum::response::IntoResponse::into_response(
            super::super::super::super::AdminError::Csrf,
        );
    };
    let request = server_admin_contract::domain_types::AdminCreateUserReq::new(
        form.display_name,
        form.login,
        form.password,
    );
    super::super::super::action_result_impl::action_result_impl(
        super::super::super::super::users::user_mutations_create::user_mutations_create(
            auth,
            super::super::super::super::AxumAdminJson(request),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
    )
}
