#[cfg(not(target_arch = "wasm32"))]
pub fn route_method_router<State, Endpoint, Marker>(
    method: crate::RouteMethod,
    endpoint: Endpoint,
) -> super::AxumRouteMethodRouter<State>
where
    State: Clone + Send + Sync + 'static,
    Endpoint: axum::handler::Handler<Marker, State> + Clone + Send + Sync + 'static,
    Marker: 'static,
{
    super::AxumRouteMethodRouter::from(match method {
        crate::RouteMethod::Connect => axum::routing::connect(endpoint),
        crate::RouteMethod::Delete => axum::routing::delete(endpoint),
        crate::RouteMethod::Get => axum::routing::get(endpoint),
        crate::RouteMethod::Head => axum::routing::head(endpoint),
        crate::RouteMethod::Options => axum::routing::options(endpoint),
        crate::RouteMethod::Patch => axum::routing::patch(endpoint),
        crate::RouteMethod::Post => axum::routing::post(endpoint),
        crate::RouteMethod::Put => axum::routing::put(endpoint),
        crate::RouteMethod::Trace => axum::routing::trace(endpoint),
    })
}
