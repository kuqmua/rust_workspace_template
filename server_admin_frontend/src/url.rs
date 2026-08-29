pub(crate) use super::admin_api_url::admin_api_url;
pub(crate) use super::admin_api_url_with_suffix::admin_api_url_with_suffix;
pub(crate) use super::admin_csr_api_url::AdminCsrApiUrl;
pub(crate) use super::admin_csr_api_url_suffix_ref::AdminCsrApiUrlSuffixRef;
pub(crate) use super::admin_http_status::AdminHttpStatus;
pub(crate) use super::admin_route_path_url::admin_route_path_url;
// Root-owned module compatibility wrappers.
pub(crate) mod admin_api_url {
    pub use super::super::admin_api_url::*;
}
pub(crate) mod admin_api_url_with_suffix {
    pub use super::super::admin_api_url_with_suffix::*;
}
pub(crate) mod admin_csr_api_url {
    pub use super::super::admin_csr_api_url::*;
}
pub(crate) mod admin_csr_api_url_suffix_ref {
    pub use super::super::admin_csr_api_url_suffix_ref::*;
}
pub(crate) mod admin_http_status {
    pub use super::super::admin_http_status::*;
}
pub(crate) mod admin_route_path_url {
    pub use super::super::admin_route_path_url::*;
}
