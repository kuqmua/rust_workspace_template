#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct HandlerPath(&'static str);
impl HandlerPath {
    #[must_use]
    pub const fn get(self) -> &'static str {
        self.0
    }
}

pub trait HandlerContract: Copy {
    fn method(self) -> super::RouteMethod;
    fn path(self) -> HandlerPath;
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct AxumHandlerMethodRouter<State>(axum::routing::MethodRouter<State>);

#[cfg(not(target_arch = "wasm32"))]
pub fn handler_method_router<State, Handler, Marker>(
    method: super::RouteMethod,
    handler: Handler,
) -> AxumHandlerMethodRouter<State>
where
    State: Clone + Send + Sync + 'static,
    Handler: axum::handler::Handler<Marker, State> + Clone + Send + Sync + 'static,
    Marker: 'static,
{
    AxumHandlerMethodRouter::from(match method {
        super::RouteMethod::Connect => axum::routing::connect(handler),
        super::RouteMethod::Delete => axum::routing::delete(handler),
        super::RouteMethod::Get => axum::routing::get(handler),
        super::RouteMethod::Head => axum::routing::head(handler),
        super::RouteMethod::Options => axum::routing::options(handler),
        super::RouteMethod::Patch => axum::routing::patch(handler),
        super::RouteMethod::Post => axum::routing::post(handler),
        super::RouteMethod::Put => axum::routing::put(handler),
        super::RouteMethod::Trace => axum::routing::trace(handler),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
    fn handler_router_supports_every_contract_method() {
        async fn handler() -> axum::http::StatusCode {
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
            let _router = super::handler_method_router::<(), _, _>(method, handler);
        });
    }

    #[test]
    fn handler_path_preserves_the_registered_static_path() {
        assert_eq!(super::HandlerPath::from("/health").get(), "/health");
    }
}
