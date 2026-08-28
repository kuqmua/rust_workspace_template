use super::{RouteMetadata, TypedRoute, UtoipaOpenApiRouteSchema};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct RouteSchemaContract {
    metadata: RouteMetadata,
    request_schema: Option<UtoipaOpenApiRouteSchema>,
    response_schema: Option<UtoipaOpenApiRouteSchema>,
}
impl RouteSchemaContract {
    #[must_use]
    pub fn from_typed_route<Route>() -> Self
    where
        Route: TypedRoute,
    {
        Self {
            metadata: Route::metadata(),
            request_schema: Route::openapi_request_schema(),
            response_schema: Route::openapi_response_schema(),
        }
    }
    #[must_use]
    pub const fn metadata(&self) -> RouteMetadata {
        self.metadata
    }
    #[must_use]
    pub const fn request_schema(&self) -> Option<&UtoipaOpenApiRouteSchema> {
        self.request_schema.as_ref()
    }
    #[must_use]
    pub const fn response_schema(&self) -> Option<&UtoipaOpenApiRouteSchema> {
        self.response_schema.as_ref()
    }
}
