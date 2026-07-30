pub(super) fn delete(id: server_admin_contract::AdminUserId) {
    if bool::from(super::super::mutation::mutation_confirmed(
        super::super::mutation::MutationConfirmationMessageRef::from("Delete this user?"),
    )) && let Ok(path) =
        super::super::http::admin_api_url(server_admin_contract::AdminRoute::DeleteUser(id))
    {
        super::super::mutation::reload_after(
            super::super::mutation::AdminMutationMethod::Delete,
            path,
            server_admin_contract::AdminNoBody,
        );
    }
}

pub(super) fn set_ban(
    id: server_admin_contract::AdminUserId,
    is_banned: server_admin_contract::AdminBool,
) {
    if let Ok(path) =
        super::super::http::admin_api_url(server_admin_contract::AdminRoute::SetUserBan(id))
    {
        super::super::mutation::reload_after(
            super::super::mutation::AdminMutationMethod::Post,
            path,
            server_admin_contract::AdminSetUserBanReq::new(server_admin_contract::AdminBool::from(
                !bool::from(is_banned),
            )),
        );
    }
}

pub(super) fn set_password(
    id: server_admin_contract::AdminUserId,
    password: server_admin_contract::AdminNewPassword,
) {
    if let Ok(path) =
        super::super::http::admin_api_url(server_admin_contract::AdminRoute::SetUserPassword(id))
    {
        super::super::mutation::reload_after(
            super::super::mutation::AdminMutationMethod::Post,
            path,
            server_admin_contract::AdminSetUserPasswordReq::new(password),
        );
    }
}

pub(super) fn set_roles(
    id: server_admin_contract::AdminUserId,
    expected: server_admin_contract::AdminRoleIds,
    selected: server_admin_contract::AdminRoleIds,
) {
    if let Ok(path) =
        super::super::http::admin_api_url(server_admin_contract::AdminRoute::SetUserRoles(id))
    {
        super::super::mutation::reload_after(
            super::super::mutation::AdminMutationMethod::Put,
            path,
            server_admin_contract::AdminSetUserRolesReq::new(expected, selected),
        );
    }
}

pub(super) fn update(
    id: server_admin_contract::AdminUserId,
    request: server_admin_contract::AdminUpdateUserReq,
) {
    if let Ok(path) =
        super::super::http::admin_api_url(server_admin_contract::AdminRoute::UpdateUser(id))
    {
        super::super::mutation::reload_after(
            super::super::mutation::AdminMutationMethod::Patch,
            path,
            request,
        );
    }
}
