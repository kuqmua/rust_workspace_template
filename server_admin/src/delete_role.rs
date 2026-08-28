#[frontend_contract::domain_types::route_error(AdminHtmlDeleteRoleError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn delete_role(
    auth: crate::AdminAuthReq,
    crate::AxumAdminForm(form): crate::AxumAdminForm<crate::RoleIdForm>,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(crate::AdminError::Validation);
    }
    crate::authenticated_action_impl::authenticated_action_impl(
        auth,
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
        |auth| {
            crate::role_mutations_delete::role_mutations_delete(
                auth,
                crate::AxumAdminPath(crate::role_path_impl::role_path_impl(form.role_id)),
            )
        },
    )
    .await
}
