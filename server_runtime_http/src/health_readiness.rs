#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct HealthReadiness {
    shared: super::SharedHealthReadinessArc,
}

impl Default for HealthReadiness {
    fn default() -> Self {
        Self {
            shared: super::SharedHealthReadinessArc::from(std::sync::Arc::from(
                std::sync::atomic::AtomicBool::new(false),
            )),
        }
    }
}

impl HealthReadiness {
    #[must_use]
    pub fn snapshot(&self) -> super::HealthSnapshot {
        let database = if self.shared.0.load(std::sync::atomic::Ordering::Acquire) {
            super::HealthComponentStatus::Ok
        } else {
            super::HealthComponentStatus::Error
        };
        super::HealthSnapshot {
            database,
            service: super::HealthComponentStatus::Ok,
        }
    }

    pub fn store_database_probe(&self, value: super::HealthProbeSucceeded) {
        self.shared
            .0
            .store(bool::from(value), std::sync::atomic::Ordering::Release);
    }
}
