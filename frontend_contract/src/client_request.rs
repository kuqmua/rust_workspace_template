#[must_use]
pub const fn client_request<Route>(
    request: Route::Request,
) -> crate::route_request::RouteRequest<Route>
where
    Route: crate::typed_route::TypedRoute,
{
    crate::route_request::RouteRequest::new(request)
}
