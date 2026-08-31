#[derive(optimal_memory_layout::OptimalMemoryLayout, serde::Serialize)]
#[serde(transparent)]
#[derive(newtype::FromInner, newtype::IntoInnerFrom)]
pub struct UtoipaCommonRoutesOpenApiDocument(utoipa::openapi::OpenApi);
impl std::fmt::Debug for UtoipaCommonRoutesOpenApiDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(constants_str::catalog::UTOIPACOMMONROUTESOPENAPIDOCUMENT)
            .finish()
    }
}
