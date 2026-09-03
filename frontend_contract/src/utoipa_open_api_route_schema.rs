#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct UtoipaOpenApiRouteSchema(utoipa::openapi::RefOr<utoipa::openapi::Schema>);
impl std::fmt::Debug for UtoipaOpenApiRouteSchema {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(constants_str::OPEN_API_ROUTE_SCHEMA)
            .finish_non_exhaustive()
    }
}
