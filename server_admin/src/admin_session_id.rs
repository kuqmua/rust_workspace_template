#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_newtype::FromInner,
)]
#[serde(from = "server_admin_core::uuid_admin_value::UuidAdminValue")]
#[derive(proc_macro_getters::Getters)]
pub struct AdminSessionId(server_admin_core::uuid_admin_value::UuidAdminValue);
