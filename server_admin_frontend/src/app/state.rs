#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, newtype::Newtype)]
#[serde(transparent)]
#[newtype(display)]
pub(super) struct Text(pub(super) String);
impl From<String> for Text {
    fn from(value: String) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, serde::Deserialize)]
pub(super) struct GitInfo {
    pub(super) commit: Option<Text>,
}
#[derive(Clone, Debug)]
pub(super) enum Page {
    Loading,
    Users(Vec<server_admin_contract::AdminUserSummary>),
    Roles(Vec<server_admin_contract::AdminRoleSummary>),
    Permissions(Vec<server_admin_contract::AdminPermissionSummary>),
    Audit(Vec<server_admin_contract::AdminAuditView>),
    Settings(server_admin_contract::AdminSettingsView),
    OpenApi(Text),
    Text(Text),
    Error(Text),
}
