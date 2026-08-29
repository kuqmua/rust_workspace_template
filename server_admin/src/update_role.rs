#[frontend_contract_macros::route_error(AdminHtmlUpdateRoleError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn update_role(
    auth: crate::admin_auth_req::AdminAuthReq,
    crate::axum_admin_form::AxumAdminForm(form): crate::axum_admin_form::AxumAdminForm<
        crate::update_role_form::UpdateRoleForm,
    >,
) -> axum::response::Response {
    let Ok(auth) = crate::form_auth_impl::form_auth_impl(auth) else {
        return axum::response::IntoResponse::into_response(crate::admin_error::AdminError::Csrf);
    };
    crate::action_result_impl::action_result_impl(
        crate::role_mutations_update::role_mutations_update(
            auth,
            crate::axum_admin_path::AxumAdminPath(crate::role_path_impl::role_path_impl(
                form.role_id,
            )),
            crate::axum_admin_json::AxumAdminJson(
                server_admin_contract::admin_update_role_req::AdminUpdateRoleReq::new(form.name),
            ),
        )
        .await,
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles,
    )
}
