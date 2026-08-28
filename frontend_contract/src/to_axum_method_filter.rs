use crate::AxumMethodFilter;

#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn to_axum_method_filter(method: crate::domain_types::RouteMethod) -> AxumMethodFilter {
    AxumMethodFilter::from(match method {
        crate::domain_types::RouteMethod::Connect => axum::routing::MethodFilter::CONNECT,
        crate::domain_types::RouteMethod::Delete => axum::routing::MethodFilter::DELETE,
        crate::domain_types::RouteMethod::Get => axum::routing::MethodFilter::GET,
        crate::domain_types::RouteMethod::Head => axum::routing::MethodFilter::HEAD,
        crate::domain_types::RouteMethod::Options => axum::routing::MethodFilter::OPTIONS,
        crate::domain_types::RouteMethod::Patch => axum::routing::MethodFilter::PATCH,
        crate::domain_types::RouteMethod::Post => axum::routing::MethodFilter::POST,
        crate::domain_types::RouteMethod::Put => axum::routing::MethodFilter::PUT,
        crate::domain_types::RouteMethod::Trace => axum::routing::MethodFilter::TRACE,
    })
}
