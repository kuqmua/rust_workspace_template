#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_debug_transparent::DebugTransparent,
    thiserror::Error,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner::IntoInner,
)]
#[error(transparent)]
#[derive(proc_macro_getters::Getters)]
pub struct SqlxAdminError(sqlx::Error);

impl From<server_admin_core::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error>
    for SqlxAdminError
{
    fn from(
        value: server_admin_core::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error,
    ) -> Self {
        Self::from(sqlx::Error::Decode(Box::new(value)))
    }
}
impl From<server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error>
    for SqlxAdminError
{
    fn from(
        value: server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error,
    ) -> Self {
        Self::from(sqlx::Error::Decode(Box::new(value)))
    }
}
