#[must_use]
pub const fn client_request<Route>(
    body: Route::Request,
) -> crate::route_request::RouteRequest<Route>
where
    Route: crate::typed_route::TypedRoute,
{
    crate::route_request::RouteRequest::new(body)
}
