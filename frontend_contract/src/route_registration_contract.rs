#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct RegisteredRoutePath(&'static str);

pub trait RouteRegistrationContract: Copy {
    fn method(self) -> super::RouteMethod;
    fn path(self) -> RegisteredRoutePath;
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct AxumRouteMethodRouter<State>(axum::routing::MethodRouter<State>);

#[cfg(not(target_arch = "wasm32"))]
pub fn route_method_router<State, Endpoint, Marker>(
    method: super::RouteMethod,
    endpoint: Endpoint,
) -> AxumRouteMethodRouter<State>
where
    State: Clone + Send + Sync + 'static,
    Endpoint: axum::handler::Handler<Marker, State> + Clone + Send + Sync + 'static,
    Marker: 'static,
{
    AxumRouteMethodRouter::from(match method {
        super::RouteMethod::Connect => axum::routing::connect(endpoint),
        super::RouteMethod::Delete => axum::routing::delete(endpoint),
        super::RouteMethod::Get => axum::routing::get(endpoint),
        super::RouteMethod::Head => axum::routing::head(endpoint),
        super::RouteMethod::Options => axum::routing::options(endpoint),
        super::RouteMethod::Patch => axum::routing::patch(endpoint),
        super::RouteMethod::Post => axum::routing::post(endpoint),
        super::RouteMethod::Put => axum::routing::put(endpoint),
        super::RouteMethod::Trace => axum::routing::trace(endpoint),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
    fn route_method_router_supports_every_contract_method() {
        async fn endpoint() -> axum::http::StatusCode {
            axum::http::StatusCode::NO_CONTENT
        }

        [
            super::super::RouteMethod::Connect,
            super::super::RouteMethod::Delete,
            super::super::RouteMethod::Get,
            super::super::RouteMethod::Head,
            super::super::RouteMethod::Options,
            super::super::RouteMethod::Patch,
            super::super::RouteMethod::Post,
            super::super::RouteMethod::Put,
            super::super::RouteMethod::Trace,
        ]
        .into_iter()
        .for_each(|method| {
            let _router = super::route_method_router::<(), _, _>(method, endpoint);
        });
    }

    #[test]
    fn registered_route_path_preserves_the_static_path() {
        assert_eq!(super::RegisteredRoutePath::from("/health").get(), "/health");
    }
}
