#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
)]
pub struct RuntimeTestConfig {
    application_base_url: crate::service_base_url::ServiceBaseUrl,
    notification_service_base_url: crate::service_base_url::ServiceBaseUrl,
}
