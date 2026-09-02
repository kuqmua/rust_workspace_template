#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub struct UtoipaOpenApiRouteSchema(utoipa::openapi::RefOr<utoipa::openapi::Schema>);
impl std::fmt::Debug for UtoipaOpenApiRouteSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(constants_str::OPEN_API_ROUTE_SCHEMA)
            .finish_non_exhaustive()
    }
}
