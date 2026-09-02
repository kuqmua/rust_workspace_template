#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInner,
    proc_macro_getters::Getters,
)]
pub(crate) struct AdminSignInJson(server_admin_contract::admin_sign_in_req::AdminSignInReq);
