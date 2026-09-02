pub(crate) async fn assignment_form_action(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    assignment_form_target: crate::assignment_form_target::AssignmentFormTarget,
) -> axum::response::Response {
    match assignment_form_target {
        crate::assignment_form_target::AssignmentFormTarget::RolePermissions(form) => {
            crate::assignment_action::assignment_action(
                admin_auth_request,
                form.get_expected_permission_ids(),
                form.get_selected(),
                crate::permission_ids_impl::permission_ids_impl,
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles,
                server_admin_contract::admin_set_role_permissions_request::AdminSetRolePermissionsRequest::new,
                crate::axum_admin_path::AxumAdminPath::from(crate::role_path_impl::role_path_impl(
                    *form.get_role_id(),
                )),
                crate::mutations_set_permissions::mutations_set_permissions,
            )
            .await
        }
        crate::assignment_form_target::AssignmentFormTarget::UserRoles(form) => {
            crate::assignment_action::assignment_action(
                admin_auth_request,
                form.get_expected_role_ids(),
                form.get_selected(),
                crate::role_ids_impl::role_ids_impl,
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users,
                server_admin_contract::admin_set_user_roles_request::AdminSetUserRolesRequest::new,
                crate::axum_admin_path::AxumAdminPath::from(crate::user_path_impl::user_path_impl(
                    *form.get_user_id(),
                )),
                crate::mutations_set_roles::mutations_set_roles,
            )
            .await
        }
    }
}
