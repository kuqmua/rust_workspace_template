use super::{RouteMetadata, TypedRoute};

#[must_use]
pub fn server_route_metadata<Route>() -> RouteMetadata
where
    Route: TypedRoute,
{
    Route::metadata()
}
