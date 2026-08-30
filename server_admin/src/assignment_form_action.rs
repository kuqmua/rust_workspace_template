pub(crate) async fn assignment_form_action(
    auth: crate::admin_auth_req::AdminAuthReq,
    target: crate::assignment_form_target::AssignmentFormTarget,
) -> axum::response::Response {
    match target {
        crate::assignment_form_target::AssignmentFormTarget::RolePermissions(form) => {
            crate::assignment_action::assignment_action(
                auth,
                &form.expected_permission_ids,
                form.selected,
                crate::permission_ids_impl::permission_ids_impl,
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles,
                server_admin_contract::admin_set_role_permissions_req::AdminSetRolePermissionsReq::new,
                crate::axum_admin_path::AxumAdminPath(crate::role_path_impl::role_path_impl(
                    form.role_id,
                )),
                crate::mutations_set_permissions::mutations_set_permissions,
            )
            .await
        }
        crate::assignment_form_target::AssignmentFormTarget::UserRoles(form) => {
            crate::assignment_action::assignment_action(
                auth,
                &form.expected_role_ids,
                form.selected,
                crate::role_ids_impl::role_ids_impl,
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users,
                server_admin_contract::admin_set_user_roles_req::AdminSetUserRolesReq::new,
                crate::axum_admin_path::AxumAdminPath(crate::user_path_impl::user_path_impl(
                    form.user_id,
                )),
                crate::mutations_set_roles::mutations_set_roles,
            )
            .await
        }
    }
}
