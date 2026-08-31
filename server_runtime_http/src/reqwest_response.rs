#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct ReqwestResponse(reqwest::Response);

impl ReqwestResponse {
    pub(crate) fn into_inner(self) -> reqwest::Response {
        self.0
    }
}
