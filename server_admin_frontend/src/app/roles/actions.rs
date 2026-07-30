pub(super) fn delete(id: server_admin_contract::AdminRoleId) {
    if bool::from(super::super::mutation::mutation_confirmed(
        super::super::mutation::MutationConfirmationMessageRef::from("Delete this role?"),
    )) && let Ok(path) =
        super::super::http::admin_api_url(server_admin_contract::AdminRoute::DeleteRole(id))
    {
        super::super::mutation::reload_after(
            super::super::mutation::AdminMutationMethod::Delete,
            path,
            server_admin_contract::AdminNoBody,
        );
    }
}

pub(super) fn set_permissions(
    id: server_admin_contract::AdminRoleId,
    expected: server_admin_contract::AdminPermissionIds,
    selected: server_admin_contract::AdminPermissionIds,
) {
    if let Ok(path) =
        super::super::http::admin_api_url(server_admin_contract::AdminRoute::SetRolePermissions(id))
    {
        super::super::mutation::reload_after(
            super::super::mutation::AdminMutationMethod::Put,
            path,
            server_admin_contract::AdminSetRolePermissionsReq::new(expected, selected),
        );
    }
}

pub(super) fn update(
    id: server_admin_contract::AdminRoleId,
    request: server_admin_contract::AdminUpdateRoleReq,
) {
    if let Ok(path) =
        super::super::http::admin_api_url(server_admin_contract::AdminRoute::UpdateRole(id))
    {
        super::super::mutation::reload_after(
            super::super::mutation::AdminMutationMethod::Patch,
            path,
            request,
        );
    }
}
