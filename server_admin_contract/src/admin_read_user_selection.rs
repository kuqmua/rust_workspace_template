#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    proc_macro_newtype_from_inner::FromInner,
    utoipa::ToSchema,
)]
#[serde(from = "[crate::admin_read_user_column::AdminReadUserColumn; 4]")]
pub struct AdminReadUserSelection([crate::admin_read_user_column::AdminReadUserColumn; 4]);
impl Default for AdminReadUserSelection {
    fn default() -> Self {
        Self::from([
            crate::admin_read_user_column::AdminReadUserColumn::Id(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_user_column::AdminReadUserColumn::Login(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_user_column::AdminReadUserColumn::DisplayName(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_user_column::AdminReadUserColumn::IsBanned(
                crate::admin_no_body::AdminNoBody,
            ),
        ])
    }
}
