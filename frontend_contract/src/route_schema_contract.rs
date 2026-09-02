#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct RouteSchemaContract {
    #[getters(copy)]
    metadata: crate::route_metadata::RouteMetadata,
    #[getters(skip)]
    request_schema: Option<crate::utoipa_open_api_route_schema::UtoipaOpenApiRouteSchema>,
    #[getters(skip)]
    response_schema: Option<crate::utoipa_open_api_route_schema::UtoipaOpenApiRouteSchema>,
}
impl RouteSchemaContract {
    #[must_use]
    pub fn from_typed_route<Route>() -> Self
    where
        Route: crate::typed_route::TypedRoute,
    {
        Self {
            metadata: Route::metadata(),
            request_schema: Route::openapi_request_schema(),
            response_schema: Route::openapi_response_schema(),
        }
    }

    #[must_use]
    pub const fn request_schema(
        &self,
    ) -> Option<&crate::utoipa_open_api_route_schema::UtoipaOpenApiRouteSchema> {
        self.request_schema.as_ref()
    }
    #[must_use]
    pub const fn response_schema(
        &self,
    ) -> Option<&crate::utoipa_open_api_route_schema::UtoipaOpenApiRouteSchema> {
        self.response_schema.as_ref()
    }
}
