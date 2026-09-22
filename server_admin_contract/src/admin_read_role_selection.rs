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
#[serde(from = "[crate::admin_read_role_column::AdminReadRoleColumn; 5]")]
pub struct AdminReadRoleSelection([crate::admin_read_role_column::AdminReadRoleColumn; 5]);
impl Default for AdminReadRoleSelection {
    fn default() -> Self {
        let columns = [
            crate::admin_read_role_column::AdminReadRoleColumn::Id(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_role_column::AdminReadRoleColumn::Name(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_role_column::AdminReadRoleColumn::IsSystem(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_role_column::AdminReadRoleColumn::CreatedAt(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_role_column::AdminReadRoleColumn::UpdatedAt(
                crate::admin_no_body::AdminNoBody,
            ),
        ];
        Self::from(columns)
    }
}
