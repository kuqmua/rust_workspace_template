#[derive(
    Debug,
    Clone,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct AxumCommonRoutes(axum::Router);
