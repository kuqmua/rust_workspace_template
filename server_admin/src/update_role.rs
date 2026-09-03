#[proc_macro_frontend_contract::route_error(AdminHtmlUpdateRoleError)]
#[allow(
    clippy::single_call_fn,
    reason = "update role remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn update_role(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_form: crate::axum_admin_form::AxumAdminForm<crate::update_role_form::UpdateRoleForm>,
) -> axum::response::Response {
    let Ok(auth) = crate::form_auth_impl::form_auth_impl(admin_auth_request) else {
        return axum::response::IntoResponse::into_response(crate::admin_error::AdminError::Csrf);
    };
    crate::action_result_impl::action_result_impl(
        crate::role_mutations_update::role_mutations_update(
            auth,
            crate::axum_admin_path::AxumAdminPath::from(crate::role_path_impl::role_path_impl(
                *axum_admin_form.get_role_id(),
            )),
            crate::axum_admin_json::AxumAdminJson::from(
                server_admin_contract::admin_update_role_request::AdminUpdateRoleRequest::new(
                    axum_admin_form.get_name().clone(),
                ),
            ),
        )
        .await,
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles,
    )
}
