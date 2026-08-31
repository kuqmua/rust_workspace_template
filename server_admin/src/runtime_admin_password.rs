#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DebugRedacted,
    newtype::FromInner,
    newtype::IntoInner,
    serde::Deserialize,
)]
#[serde(try_from = "String")]
#[derive(generate_accessor::Getters)]
pub struct RuntimeAdminPassword(server_admin_core::secrecy_admin_string::SecrecyAdminString);
