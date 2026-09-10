#[proc_macro_frontend_contract_route_error::route_error(AdminHtmlCreateRoleError)]
#[allow(
    clippy::single_call_fn,
    reason = "create role remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn create_role(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_form: crate::axum_admin_form::AxumAdminForm<crate::create_role_form::CreateRoleForm>,
) -> axum::response::Response {
    let Ok(auth) = crate::form_auth_impl::form_auth_impl(admin_auth_request) else {
        return axum::response::IntoResponse::into_response(crate::admin_error::AdminError::Csrf);
    };
    crate::action_result_impl::action_result_impl(
        async {
            let request = server_admin_contract::admin_create_roles_request::AdminCreateRolesRequest::try_from(vec![
                server_admin_contract::admin_create_role_request::AdminCreateRoleRequest::new(axum_admin_form.get_name().clone()),
            ]).map_err(|server_admin_contract::admin_collection_error::AdminCollectionError::TooLong| crate::admin_error::AdminError::Validation)?;
            crate::role_mutations_create_many::role_mutations_create_many(auth, request).await.map(|identifiers| {
                crate::axum_admin_response::AxumAdminResponse::from(axum::response::IntoResponse::into_response(axum::Json(identifiers)))
            })
        }.await,
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles,
    )
}
