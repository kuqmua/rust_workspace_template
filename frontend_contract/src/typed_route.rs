use super::{
    RouteMetadata, RouteRequestBody, RouteTransport, UtoipaOpenApiComponentsRefMut,
    UtoipaOpenApiPathParameter, UtoipaOpenApiRouteSchema,
};

pub trait TypedRoute: Sized {
    type Request: serde::Serialize + serde::de::DeserializeOwned;
    type Response: serde::Serialize + serde::de::DeserializeOwned;
    type Transport: RouteTransport;
    fn metadata() -> RouteMetadata;
    #[must_use]
    fn openapi_request_schema() -> Option<UtoipaOpenApiRouteSchema> {
        None
    }
    #[must_use]
    fn openapi_request_body_schema() -> Option<UtoipaOpenApiRouteSchema> {
        None
    }
    #[must_use]
    fn openapi_response_schema() -> Option<UtoipaOpenApiRouteSchema> {
        None
    }
    #[must_use]
    fn openapi_error_response_schema(
        _status: crate::RouteErrorStatus,
    ) -> Option<UtoipaOpenApiRouteSchema> {
        Some(UtoipaOpenApiRouteSchema::from(
            <crate::ApiProblem as utoipa::PartialSchema>::schema(),
        ))
    }
    #[must_use]
    fn openapi_path_parameter() -> Option<UtoipaOpenApiPathParameter> {
        None
    }
    #[must_use]
    fn request_body() -> RouteRequestBody {
        RouteRequestBody::Absent
    }
    fn register_openapi_schemas(_components: &mut UtoipaOpenApiComponentsRefMut<'_>) {}
}
