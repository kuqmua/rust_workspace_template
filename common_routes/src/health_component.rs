#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_new::New,
)]
#[constructor(pub(crate))]
pub struct HealthComponent {
    kind: crate::health_component_kind::HealthComponentKind,
    #[getters(copy)]
    status: crate::health_status::HealthStatus,
}
