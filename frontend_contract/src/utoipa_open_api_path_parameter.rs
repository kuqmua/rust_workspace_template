#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct UtoipaOpenApiPathParameter(utoipa::openapi::path::Parameter);
impl std::fmt::Debug for UtoipaOpenApiPathParameter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(constants_str::UTOIPAOPENAPIPATHPARAMETER)
            .finish_non_exhaustive()
    }
}
