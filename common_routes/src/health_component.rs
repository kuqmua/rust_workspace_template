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
)]
pub struct HealthComponent {
    kind: crate::health_component_kind::HealthComponentKind,
    #[getters(copy)]
    status: crate::health_status::HealthStatus,
}
impl HealthComponent {
    #[must_use]
    pub(crate) const fn new(
        kind: crate::health_component_kind::HealthComponentKind,
        status: crate::health_status::HealthStatus,
    ) -> Self {
        Self { kind, status }
    }
}
