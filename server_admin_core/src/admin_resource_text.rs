use crate::domain_types::UuidAdminValue;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum AdminResourceText {
    PositiveI64(server_admin_contract::domain_types::PositiveNonZeroI64),
    SystemSettings,
    Uuid(UuidAdminValue),
}
