#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn to_axum_method_filter(
    route_method: crate::route_method::RouteMethod,
) -> crate::axum_method_filter::AxumMethodFilter {
    crate::axum_method_filter::AxumMethodFilter::from(match route_method {
        crate::route_method::RouteMethod::Connect => axum::routing::MethodFilter::CONNECT,
        crate::route_method::RouteMethod::Delete => axum::routing::MethodFilter::DELETE,
        crate::route_method::RouteMethod::Get => axum::routing::MethodFilter::GET,
        crate::route_method::RouteMethod::Head => axum::routing::MethodFilter::HEAD,
        crate::route_method::RouteMethod::Options => axum::routing::MethodFilter::OPTIONS,
        crate::route_method::RouteMethod::Patch => axum::routing::MethodFilter::PATCH,
        crate::route_method::RouteMethod::Post => axum::routing::MethodFilter::POST,
        crate::route_method::RouteMethod::Put => axum::routing::MethodFilter::PUT,
        crate::route_method::RouteMethod::Trace => axum::routing::MethodFilter::TRACE,
    })
}
