#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
    serde::Serialize,
)]
#[constructor(pub(crate))]
pub struct HealthSnapshot {
    #[getters(copy)]
    database: crate::health_component_status::HealthComponentStatus,
    #[getters(copy)]
    service: crate::health_component_status::HealthComponentStatus,
}
