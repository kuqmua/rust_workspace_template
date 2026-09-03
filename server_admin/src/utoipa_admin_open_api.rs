#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
pub struct UtoipaAdminOpenApi(utoipa::openapi::OpenApi);
impl std::fmt::Debug for UtoipaAdminOpenApi {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(constants_str::UTOIPAADMINOPENAPI)
    }
}
