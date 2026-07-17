#[derive(Clone, Debug, Default, newtype::BoundedString, newtype::Display)]
#[bounded_string(max = 16_777_216usize, serde)]
pub(super) struct Text(pub(super) String);
#[derive(Clone, Debug, serde::Deserialize)]
pub(super) struct GitInfo {
    pub(super) commit: Option<Text>,
}
#[derive(Clone, Debug)]
pub(super) enum Page {
    Loading,
    Dashboard(server_admin_contract::AdminDashboardView),
    Profile,
    Users(
        Vec<server_admin_contract::AdminUserSummary>,
        Vec<server_admin_contract::AdminRoleSummary>,
        server_admin_contract::AdminPageTotal,
    ),
    Roles(
        Vec<server_admin_contract::AdminRoleSummary>,
        Vec<server_admin_contract::AdminPermissionSummary>,
        server_admin_contract::AdminPageTotal,
    ),
    Permissions(
        Vec<server_admin_contract::AdminPermissionSummary>,
        server_admin_contract::AdminPageTotal,
    ),
    Audit(
        Vec<server_admin_contract::AdminAuditView>,
        Option<server_admin_contract::AdminAuditCursor>,
    ),
    Settings(server_admin_contract::AdminSettingsView),
    Sessions(Vec<server_admin_contract::AdminSessionView>),
    OpenApi(Text),
    Text(Text),
    Error(Text),
}
