pub(super) fn request(
    display_name: Option<server_admin_contract::AdminDisplayName>,
    login: Option<server_admin_contract::AdminLogin>,
) -> server_admin_contract::AdminUpdateUserReq {
    server_admin_contract::AdminUpdateUserReq::new(display_name, login)
}
