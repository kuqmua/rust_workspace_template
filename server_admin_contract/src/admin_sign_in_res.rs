use super::AuthenticatedAdmin;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminSignInRes {
    #[contract_struct_api(borrow)]
    user: AuthenticatedAdmin,
}
