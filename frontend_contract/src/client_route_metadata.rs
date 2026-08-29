#[must_use]
pub fn client_route_metadata<Route>() -> crate::route_metadata::RouteMetadata
where
    Route: crate::typed_route::TypedRoute,
{
    Route::metadata()
}
