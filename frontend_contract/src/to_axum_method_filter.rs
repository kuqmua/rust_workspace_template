use super::super::AxumMethodFilter;

#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn to_axum_method_filter(method: crate::domain_types::HttpMethod) -> AxumMethodFilter {
    AxumMethodFilter::from(match method {
        crate::domain_types::HttpMethod::Connect => axum::routing::MethodFilter::CONNECT,
        crate::domain_types::HttpMethod::Delete => axum::routing::MethodFilter::DELETE,
        crate::domain_types::HttpMethod::Get => axum::routing::MethodFilter::GET,
        crate::domain_types::HttpMethod::Head => axum::routing::MethodFilter::HEAD,
        crate::domain_types::HttpMethod::Options => axum::routing::MethodFilter::OPTIONS,
        crate::domain_types::HttpMethod::Patch => axum::routing::MethodFilter::PATCH,
        crate::domain_types::HttpMethod::Post => axum::routing::MethodFilter::POST,
        crate::domain_types::HttpMethod::Put => axum::routing::MethodFilter::PUT,
        crate::domain_types::HttpMethod::Trace => axum::routing::MethodFilter::TRACE,
    })
}
