use super::{RouteMetadata, TypedRoute};

#[must_use]
pub fn client_route_metadata<Route>() -> RouteMetadata
where
    Route: TypedRoute,
{
    Route::metadata()
}
