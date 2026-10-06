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

#[cfg(test)]
mod tests {
    #[test]
    fn test_common_openapi_document_debug_is_opaque_and_ownership_transfer_preserves_json() {
        let document = crate::common_routes_open_api::CommonRoutesOpenApi::open_api();
        assert_eq!(
            format!("{document:?}"),
            constants_str::UTOIPACOMMONROUTESOPENAPIDOCUMENT
        );
        let expected_result = serde_json::to_value(&document);
        let native_document = utoipa::openapi::OpenApi::from(document);
        assert!(!native_document.paths.paths.is_empty());
        assert!(expected_result.is_ok_and(|expected| {
            serde_json::to_value(native_document).is_ok_and(|value| value == expected)
        }));
    }
}
