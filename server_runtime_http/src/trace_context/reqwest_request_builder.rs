#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub struct ReqwestRequestBuilder(pub(super) reqwest::RequestBuilder);
