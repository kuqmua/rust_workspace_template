#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract_macros::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminSignInRes {
    #[contract_struct_api(borrow)]
    user: crate::authenticated_admin::AuthenticatedAdmin,
}
