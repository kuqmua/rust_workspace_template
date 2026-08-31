#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::FromInner,
    newtype::IntoInner,
    generate_accessor::Getters,
)]
pub(crate) struct AdminSignInJson(server_admin_contract::admin_sign_in_req::AdminSignInReq);
