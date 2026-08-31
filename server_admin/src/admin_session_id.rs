#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::FromInner,
)]
#[serde(from = "server_admin_core::uuid_admin_value::UuidAdminValue")]
#[derive(generate_accessor::Getters)]
pub struct AdminSessionId(server_admin_core::uuid_admin_value::UuidAdminValue);
