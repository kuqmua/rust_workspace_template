#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, serde::Serialize)]
#[serde(transparent)]
#[derive(
    proc_macro_newtype_from_inner::FromInner, proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct UtoipaCommonRoutesOpenApiDocument(utoipa::openapi::OpenApi);
impl std::fmt::Debug for UtoipaCommonRoutesOpenApiDocument {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple(constants_str::UTOIPACOMMONROUTESOPENAPIDOCUMENT)
            .finish()
    }
}
