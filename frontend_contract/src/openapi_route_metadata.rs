#[must_use]
pub fn openapi_route_metadata<Route>() -> crate::route_metadata::RouteMetadata
where
    Route: crate::typed_route::TypedRoute,
{
    Route::metadata()
}
