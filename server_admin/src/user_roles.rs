#[frontend_contract::domain_types::route_error(AdminHtmlUserRolesError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn user_roles(
    auth: crate::AdminAuthReq,
    crate::AxumAdminForm(form): crate::AxumAdminForm<crate::UserRolesForm>,
) -> axum::response::Response {
    crate::assignment_action(
        auth,
        &form.expected_role_ids,
        form.selected,
        crate::role_ids_impl::role_ids_impl,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
        server_admin_contract::domain_types::AdminSetUserRolesReq::new,
        crate::AxumAdminPath(crate::user_path_impl::user_path_impl(form.user_id)),
        crate::mutations_set_roles::mutations_set_roles,
    )
    .await
}
