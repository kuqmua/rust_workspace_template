#[frontend_contract::route_error(AdminHtmlUpdateRoleError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn update_role(
    auth: crate::AdminAuthReq,
    crate::AxumAdminForm(form): crate::AxumAdminForm<crate::UpdateRoleForm>,
) -> axum::response::Response {
    let Ok(auth) = crate::form_auth_impl::form_auth_impl(auth) else {
        return axum::response::IntoResponse::into_response(crate::AdminError::Csrf);
    };
    crate::action_result_impl::action_result_impl(
        crate::role_mutations_update::role_mutations_update(
            auth,
            crate::AxumAdminPath(crate::role_path_impl::role_path_impl(form.role_id)),
            crate::AxumAdminJson(
                server_admin_contract::domain_types::AdminUpdateRoleReq::new(form.name),
            ),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
    )
}
