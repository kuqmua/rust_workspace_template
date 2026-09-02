#[derive(proc_macro_getters::Getters, proc_macro_new::New)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct ReqwestClientPolicy {
    #[getters(copy)]
    connect_timeout: super::reqwest_connect_timeout_duration::ReqwestConnectTimeoutDuration,
    #[getters(copy)]
    request_timeout: super::reqwest_request_timeout_duration::ReqwestRequestTimeoutDuration,
}
