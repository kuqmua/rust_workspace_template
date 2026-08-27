#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct ReqwestResponse(pub(super) reqwest::Response);
