#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{self:?}")]
pub struct AdminAuthPositiveValueError;
impl From<server_admin_contract::domain_types::AdminIdTryFromI64Error>
    for AdminAuthPositiveValueError
{
    fn from(_value: server_admin_contract::domain_types::AdminIdTryFromI64Error) -> Self {
        Self
    }
}
