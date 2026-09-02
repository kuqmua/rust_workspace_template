pub trait TypedRoute: Sized {
    type Request: serde::Serialize + serde::de::DeserializeOwned;
    type Response: serde::Serialize + serde::de::DeserializeOwned;
    type Transport: crate::route_transport::RouteTransport;
    fn metadata() -> crate::route_metadata::RouteMetadata;
    #[must_use]
    fn openapi_request_schema()
    -> Option<crate::utoipa_open_api_route_schema::UtoipaOpenApiRouteSchema> {
        None
    }
    #[must_use]
    fn openapi_request_body_schema()
    -> Option<crate::utoipa_open_api_route_schema::UtoipaOpenApiRouteSchema> {
        None
    }
    #[must_use]
    fn openapi_response_schema()
    -> Option<crate::utoipa_open_api_route_schema::UtoipaOpenApiRouteSchema> {
        None
    }
    #[must_use]
    #[allow(
        unused_variables,
        reason = "the default trait hook preserves the repository type-based parameter name"
    )]
    fn openapi_error_response_schema(
        route_error_status: crate::route_error_status::RouteErrorStatus,
    ) -> Option<crate::utoipa_open_api_route_schema::UtoipaOpenApiRouteSchema> {
        Some(
            crate::utoipa_open_api_route_schema::UtoipaOpenApiRouteSchema::from(
                <crate::api_problem::ApiProblem as utoipa::PartialSchema>::schema(),
            ),
        )
    }
    #[must_use]
    fn openapi_path_parameter()
    -> Option<crate::utoipa_open_api_path_parameter::UtoipaOpenApiPathParameter> {
        None
    }
    #[must_use]
    fn request_body() -> crate::route_request_body::RouteRequestBody {
        crate::route_request_body::RouteRequestBody::Absent
    }
    #[allow(
        unused_variables,
        reason = "the default trait hook preserves the repository type-based parameter name"
    )]
    fn register_openapi_schemas(
        utoipa_open_api_components_ref_mut: &mut crate::utoipa_open_api_components_ref_mut::UtoipaOpenApiComponentsRefMut<
            '_,
        >,
    ) {
    }
}
