#[derive(
    Debug,
    Clone,
    Copy,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct AxumHttpUriRef<'uri_lt>(&'uri_lt axum::http::Uri);
