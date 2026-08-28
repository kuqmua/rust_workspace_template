#[frontend_contract::domain_types::route_error(AdminHtmlCreateRoleError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn create_role(
    auth: crate::AdminAuthReq,
    crate::AxumAdminForm(form): crate::AxumAdminForm<crate::CreateRoleForm>,
) -> axum::response::Response {
    let Ok(auth) = crate::form_auth_impl::form_auth_impl(auth) else {
        return axum::response::IntoResponse::into_response(crate::AdminError::Csrf);
    };
    crate::action_result_impl::action_result_impl(
        crate::role_mutations_create::role_mutations_create(
            auth,
            crate::AxumAdminJson(
                server_admin_contract::domain_types::AdminCreateRoleReq::new(form.name),
            ),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
    )
}
