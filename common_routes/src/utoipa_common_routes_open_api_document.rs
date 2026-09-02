#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, serde::Serialize)]
#[serde(transparent)]
#[derive(proc_macro_newtype::FromInner, proc_macro_newtype::IntoInnerFrom)]
pub struct UtoipaCommonRoutesOpenApiDocument(utoipa::openapi::OpenApi);
impl std::fmt::Debug for UtoipaCommonRoutesOpenApiDocument {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple(constants_str::UTOIPACOMMONROUTESOPENAPIDOCUMENT)
            .finish()
    }
}
