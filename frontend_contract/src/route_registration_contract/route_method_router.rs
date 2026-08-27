#[cfg(not(target_arch = "wasm32"))]
pub fn route_method_router<State, Endpoint, Marker>(
    method: super::super::RouteMethod,
    endpoint: Endpoint,
) -> super::AxumRouteMethodRouter<State>
where
    State: Clone + Send + Sync + 'static,
    Endpoint: axum::handler::Handler<Marker, State> + Clone + Send + Sync + 'static,
    Marker: 'static,
{
    super::AxumRouteMethodRouter::from(match method {
        super::super::RouteMethod::Connect => axum::routing::connect(endpoint),
        super::super::RouteMethod::Delete => axum::routing::delete(endpoint),
        super::super::RouteMethod::Get => axum::routing::get(endpoint),
        super::super::RouteMethod::Head => axum::routing::head(endpoint),
        super::super::RouteMethod::Options => axum::routing::options(endpoint),
        super::super::RouteMethod::Patch => axum::routing::patch(endpoint),
        super::super::RouteMethod::Post => axum::routing::post(endpoint),
        super::super::RouteMethod::Put => axum::routing::put(endpoint),
        super::super::RouteMethod::Trace => axum::routing::trace(endpoint),
    })
}
