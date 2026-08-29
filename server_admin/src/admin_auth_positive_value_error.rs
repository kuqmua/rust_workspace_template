#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{self:?}")]
pub struct AdminAuthPositiveValueError;
impl From<server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error>
    for AdminAuthPositiveValueError
{
    fn from(
        _value: server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error,
    ) -> Self {
        Self
    }
}
