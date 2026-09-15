#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
#[getters(get_mut)]
pub struct AxumAdminResponse(axum::response::Response);
impl axum::response::IntoResponse for AxumAdminResponse {
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::from(self)
    }
}
