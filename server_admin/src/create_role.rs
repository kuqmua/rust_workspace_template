#[frontend_contract_macros::route_error(AdminHtmlCreateRoleError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn create_role(
    auth: crate::admin_auth_req::AdminAuthReq,
    form: crate::axum_admin_form::AxumAdminForm<crate::create_role_form::CreateRoleForm>,
) -> axum::response::Response {
    let Ok(auth) = crate::form_auth_impl::form_auth_impl(auth) else {
        return axum::response::IntoResponse::into_response(crate::admin_error::AdminError::Csrf);
    };
    crate::action_result_impl::action_result_impl(
        crate::role_mutations_create::role_mutations_create(
            auth,
            crate::axum_admin_json::AxumAdminJson::from(
                server_admin_contract::admin_create_role_req::AdminCreateRoleReq::new(
                    form.get_name().clone(),
                ),
            ),
        )
        .await,
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles,
    )
}
