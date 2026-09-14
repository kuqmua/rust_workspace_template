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
                |expected_permission_ids, permission_ids| {
                    let updates = server_admin_contract::admin_role_updates::AdminRoleUpdates::try_from(vec![
                        server_admin_contract::admin_role_update::AdminRoleUpdate::new(
                            server_admin_contract::admin_update_role_request::AdminUpdateRoleRequest::new(
                                None,
                                Some(server_admin_contract::admin_set_role_permissions_request::AdminSetRolePermissionsRequest::new(
                                    expected_permission_ids,
                                    permission_ids,
                                )),
                            ),
                            server_admin_contract::admin_role_filter::AdminRoleFilter::new(
                                Some(*form.get_role_id()),
                                None,
                                None,
                            ),
                        ),
                    ])
                    .map_err(
                        |server_admin_contract::admin_collection_error::AdminCollectionError::TooLong| {
                            crate::admin_error::AdminError::Validation
                        },
                    )?;
                    Ok(server_admin_contract::admin_update_roles_request::AdminUpdateRolesRequest::new(updates))
                },
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles,
                |admin_auth_request, _admin_frontend_path, axum_admin_json| {
                    crate::dispatch_filtered_update::dispatch_filtered_update::<_, crate::admin_error::AdminError>(
                        admin_auth_request,
                        axum_admin_json,
                    )
                },
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
                |expected, selected| Ok(server_admin_contract::admin_update_user_request::AdminUpdateUserRequest::new(
                    None, None, None, Some(expected), Some(selected), None,
                )),
                crate::axum_admin_path::AxumAdminPath::from(crate::user_path_impl::user_path_impl(
                    *form.get_user_id(),
                )),
                crate::user_mutations_update::user_mutations_update,
            )
            .await
        }
    }
}
