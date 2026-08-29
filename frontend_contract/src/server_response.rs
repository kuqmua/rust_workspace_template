#[must_use]
pub const fn server_response<Route>(
    body: Route::Response,
) -> crate::route_response::RouteResponse<Route>
where
    Route: crate::typed_route::TypedRoute,
{
    crate::route_response::RouteResponse { body }
}
