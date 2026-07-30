pub(super) fn request(
    name: server_admin_contract::AdminRoleName,
) -> server_admin_contract::AdminUpdateRoleReq {
    server_admin_contract::AdminUpdateRoleReq::new(name)
}
