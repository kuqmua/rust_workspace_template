#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum AdminResourceText {
    PositiveI64(server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64),
    SystemSettings,
    Uuid(crate::uuid_admin_value::UuidAdminValue),
}
