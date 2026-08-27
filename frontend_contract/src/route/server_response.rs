use super::{RouteResponse, TypedRoute};

#[must_use]
pub const fn server_response<Route>(body: Route::Response) -> RouteResponse<Route>
where
    Route: TypedRoute,
{
    RouteResponse { body }
}
