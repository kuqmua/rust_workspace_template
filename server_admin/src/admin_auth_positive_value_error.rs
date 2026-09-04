#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum AdminAuthPositiveValueError {
    #[error("{self:?}")]
    Zero,
}
impl From<server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error>
    for AdminAuthPositiveValueError
{
    fn from(
        value: server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error,
    ) -> Self {
        let _: server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error = value;
        Self::Zero
    }
}
