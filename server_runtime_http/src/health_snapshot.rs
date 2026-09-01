#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
    serde::Serialize,
)]
#[constructor(pub(crate))]
pub struct HealthSnapshot {
    #[getters(copy)]
    database: crate::health_component_status::HealthComponentStatus,
    #[getters(copy)]
    service: crate::health_component_status::HealthComponentStatus,
}
