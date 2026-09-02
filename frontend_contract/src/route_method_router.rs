#[cfg(not(target_arch = "wasm32"))]
pub fn route_method_router<State, Endpoint, Marker>(
    route_method: crate::route_method::RouteMethod,
    endpoint: Endpoint,
) -> crate::axum_route_method_router::AxumRouteMethodRouter<State>
where
    State: Clone + Send + Sync + 'static,
    Endpoint: axum::handler::Handler<Marker, State> + Clone + Send + Sync + 'static,
    Marker: 'static,
{
    crate::axum_route_method_router::AxumRouteMethodRouter::from(match route_method {
        crate::route_method::RouteMethod::Connect => axum::routing::connect(endpoint),
        crate::route_method::RouteMethod::Delete => axum::routing::delete(endpoint),
        crate::route_method::RouteMethod::Get => axum::routing::get(endpoint),
        crate::route_method::RouteMethod::Head => axum::routing::head(endpoint),
        crate::route_method::RouteMethod::Options => axum::routing::options(endpoint),
        crate::route_method::RouteMethod::Patch => axum::routing::patch(endpoint),
        crate::route_method::RouteMethod::Post => axum::routing::post(endpoint),
        crate::route_method::RouteMethod::Put => axum::routing::put(endpoint),
        crate::route_method::RouteMethod::Trace => axum::routing::trace(endpoint),
    })
}
