use super::{RouteRequest, TypedRoute};

#[must_use]
pub const fn client_request<Route>(body: Route::Request) -> RouteRequest<Route>
where
    Route: TypedRoute,
{
    RouteRequest::new(body)
}
