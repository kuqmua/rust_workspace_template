// The owner module retains lint-sensitive semantics from the original implementation.

#[allow(
    clippy::single_call_fn,
    reason = "public OpenAPI facade owns this adapter"
)]
pub(crate) fn common_routes_open_api() -> crate::domain_types::UtoipaCommonRoutesOpenApiDocument {
    crate::domain_types::UtoipaCommonRoutesOpenApiDocument::from(
        super::common_route_registry::CommonRouteRegistry::open_api(),
    )
}
