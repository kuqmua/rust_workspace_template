#[frontend_contract_macros::route_error(AdminHtmlUserRolesError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn user_roles(
    auth: crate::admin_auth_req::AdminAuthReq,
    crate::axum_admin_form::AxumAdminForm(form): crate::axum_admin_form::AxumAdminForm<
        crate::user_roles_form::UserRolesForm,
    >,
) -> axum::response::Response {
    crate::assignment_action::assignment_action(
        auth,
        &form.expected_role_ids,
        form.selected,
        crate::role_ids_impl::role_ids_impl,
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Users,
        server_admin_contract::admin_set_user_roles_req::AdminSetUserRolesReq::new,
        crate::axum_admin_path::AxumAdminPath(crate::user_path_impl::user_path_impl(form.user_id)),
        crate::mutations_set_roles::mutations_set_roles,
    )
    .await
}
