#[proc_macro_frontend_contract_route_error::route_error(AdminHtmlUpdateRoleError)]
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
    let updates = [
        server_admin_contract::admin_role_update::AdminRoleUpdate::new(
            server_admin_contract::admin_update_role_request::AdminUpdateRoleRequest::new(
                axum_admin_form.get_name().clone(),
            ),
            server_admin_contract::admin_role_filter::AdminRoleFilter::new(
                Some(*axum_admin_form.get_role_id()),
                None,
                None,
            ),
        ),
    ];
    crate::action_result_impl::action_result_impl(
        crate::role_mutations_update_many::role_mutations_update_many(
            auth,
            crate::admin_role_update_slice::AdminRoleUpdateSlice::from(updates.as_slice()),
        )
        .await,
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles,
    )
}
