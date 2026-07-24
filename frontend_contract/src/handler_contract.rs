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

#[derive(Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct AxumHandlerMethodRouter<State>(axum::routing::MethodRouter<State>);

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
