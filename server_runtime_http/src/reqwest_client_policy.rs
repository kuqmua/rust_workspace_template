#[derive(generate_accessor::Getters, generate_constructor::New)]
#[getters(bare)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct ReqwestClientPolicy {
    #[getters(copy)]
    connect_timeout: super::reqwest_connect_timeout_duration::ReqwestConnectTimeoutDuration,
    #[getters(copy)]
    request_timeout: super::reqwest_request_timeout_duration::ReqwestRequestTimeoutDuration,
}
