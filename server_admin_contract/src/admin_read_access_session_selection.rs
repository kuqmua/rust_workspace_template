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
#[serde(from = "[crate::admin_read_access_session_column::AdminReadAccessSessionColumn; 5]")]
pub struct AdminReadAccessSessionSelection(
    [crate::admin_read_access_session_column::AdminReadAccessSessionColumn; 5],
);

impl Default for AdminReadAccessSessionSelection {
    fn default() -> Self {
        Self::from([
            crate::admin_read_access_session_column::AdminReadAccessSessionColumn::Id(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_access_session_column::AdminReadAccessSessionColumn::UserId(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_access_session_column::AdminReadAccessSessionColumn::ExpiresAt(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_access_session_column::AdminReadAccessSessionColumn::CreatedAt(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_access_session_column::AdminReadAccessSessionColumn::RevokedAt(
                crate::admin_no_body::AdminNoBody,
            ),
        ])
    }
}
