#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::DebugRedacted,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInner,
    serde::Deserialize,
)]
#[serde(try_from = "String")]
#[derive(proc_macro_getters::Getters)]
pub struct RuntimeAdminPassword(server_admin_core::secrecy_admin_string::SecrecyAdminString);
