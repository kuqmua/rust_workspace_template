#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct HealthReadiness {
    shared: crate::shared_health_readiness_arc::SharedHealthReadinessArc,
}

impl Default for HealthReadiness {
    fn default() -> Self {
        Self {
            shared: crate::shared_health_readiness_arc::SharedHealthReadinessArc::from(
                std::sync::Arc::from(std::sync::atomic::AtomicBool::new(false)),
            ),
        }
    }
}

impl HealthReadiness {
    #[must_use]
    pub fn snapshot(&self) -> crate::health_snapshot::HealthSnapshot {
        let database = if self.shared.load(std::sync::atomic::Ordering::Acquire) {
            crate::health_component_status::HealthComponentStatus::Ok
        } else {
            crate::health_component_status::HealthComponentStatus::Error
        };
        crate::health_snapshot::HealthSnapshot::new(
            database,
            crate::health_component_status::HealthComponentStatus::Ok,
        )
    }

    pub fn store_database_probe(&self, value: crate::health_probe_succeeded::HealthProbeSucceeded) {
        self.shared
            .store(bool::from(value), std::sync::atomic::Ordering::Release);
    }
}
