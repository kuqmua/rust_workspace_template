#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
pub struct GenerationGate {
    current: super::GenerationAtomicU64,
}

impl GenerationGate {
    #[must_use]
    pub fn begin(&self) -> super::Generation {
        super::Generation::from(
            self.current
                .0
                .fetch_add(1u64, std::sync::atomic::Ordering::AcqRel)
                .saturating_add(1u64),
        )
    }

    #[must_use]
    pub fn classify(&self, generation: super::Generation) -> super::GenerationCommit {
        if self.current.0.load(std::sync::atomic::Ordering::Acquire) == generation.0 {
            super::GenerationCommit::Current
        } else {
            super::GenerationCommit::Stale
        }
    }
}
