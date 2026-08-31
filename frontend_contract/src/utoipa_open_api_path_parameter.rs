#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub struct UtoipaOpenApiPathParameter(utoipa::openapi::path::Parameter);
impl std::fmt::Debug for UtoipaOpenApiPathParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(constants_str::UTOIPAOPENAPIPATHPARAMETER)
            .finish_non_exhaustive()
    }
}
