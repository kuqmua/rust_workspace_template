#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct CommonRoutesOpenApi;
impl CommonRoutesOpenApi {
    #[must_use]
    pub fn open_api()
    -> crate::utoipa_common_routes_open_api_document::UtoipaCommonRoutesOpenApiDocument {
        crate::utoipa_common_routes_open_api_document::UtoipaCommonRoutesOpenApiDocument::from(
            crate::common_route_registry::open_api(),
        )
    }
}
