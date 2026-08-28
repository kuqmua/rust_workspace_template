use super::UtoipaCommonRoutesOpenApiDocument;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct CommonRoutesOpenApi;
impl CommonRoutesOpenApi {
    #[must_use]
    pub fn open_api() -> UtoipaCommonRoutesOpenApiDocument {
        UtoipaCommonRoutesOpenApiDocument::from(
            crate::common_route_registry::CommonRouteRegistry::open_api(),
        )
    }
}
