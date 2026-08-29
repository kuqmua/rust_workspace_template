use super::AxumMethodFilter;

#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn to_axum_method_filter(method: crate::RouteMethod) -> AxumMethodFilter {
    AxumMethodFilter::from(match method {
        crate::RouteMethod::Connect => axum::routing::MethodFilter::CONNECT,
        crate::RouteMethod::Delete => axum::routing::MethodFilter::DELETE,
        crate::RouteMethod::Get => axum::routing::MethodFilter::GET,
        crate::RouteMethod::Head => axum::routing::MethodFilter::HEAD,
        crate::RouteMethod::Options => axum::routing::MethodFilter::OPTIONS,
        crate::RouteMethod::Patch => axum::routing::MethodFilter::PATCH,
        crate::RouteMethod::Post => axum::routing::MethodFilter::POST,
        crate::RouteMethod::Put => axum::routing::MethodFilter::PUT,
        crate::RouteMethod::Trace => axum::routing::MethodFilter::TRACE,
    })
}
