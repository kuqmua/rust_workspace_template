use super::UtoipaCommonRoutesOpenApiDocument;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct CommonRoutesOpenApi;
impl CommonRoutesOpenApi {
    #[must_use]
    pub fn open_api() -> UtoipaCommonRoutesOpenApiDocument {
        crate::adapters::common_routes_open_api()
    }
}
