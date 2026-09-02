#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::AsMut,
    proc_macro_newtype::FromInner,
)]
pub struct UtoipaOpenApiRefMut<'value_lt>(&'value_lt mut utoipa::openapi::OpenApi);
impl std::fmt::Debug for UtoipaOpenApiRefMut<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(constants_str::UTOIPAOPENAPIREFMUT)
            .finish_non_exhaustive()
    }
}
