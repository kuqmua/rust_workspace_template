// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::single_call_fn)]
pub(crate) fn open_api() -> crate::domain_types::UtoipaCommonRoutesOpenApiDocument {
    crate::domain_types::UtoipaCommonRoutesOpenApiDocument::from(
        super::common_route_registry::CommonRouteRegistry::open_api(),
    )
}
