#[frontend_contract::domain_types::route_error(AdminHtmlUserRolesError)]
pub(super) async fn user_roles(
    auth: super::super::super::super::AdminAuthReq,
    super::super::super::super::AxumAdminForm(form): super::super::super::super::AxumAdminForm<
        super::super::super::forms::UserRolesForm,
    >,
) -> axum::response::Response {
    super::super::assignment_action(
        auth,
        &form.expected_role_ids,
        form.selected,
        super::super::super::role_ids_impl::role_ids_impl,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
        server_admin_contract::domain_types::AdminSetUserRolesReq::new,
        super::super::super::super::AxumAdminPath(
            super::super::super::user_path_impl::user_path_impl(form.user_id),
        ),
        super::super::super::super::users::mutations_set_roles::mutations_set_roles,
    )
    .await
}
