#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[path = "generation.rs"]
mod generation;
#[path = "generation_atomic_u64.rs"]
mod generation_atomic_u64;
#[path = "generation_commit.rs"]
mod generation_commit;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
pub struct GenerationGate {
    current: GenerationAtomicU64,
}

impl GenerationGate {
    #[must_use]
    pub fn begin(&self) -> Generation {
        Generation::from(
            self.current
                .0
                .fetch_add(1u64, std::sync::atomic::Ordering::AcqRel)
                .saturating_add(1u64),
        )
    }

    #[must_use]
    pub fn classify(&self, generation: Generation) -> GenerationCommit {
        if self.current.0.load(std::sync::atomic::Ordering::Acquire) == generation.0 {
            GenerationCommit::Current
        } else {
            GenerationCommit::Stale
        }
    }
}

pub use generation::Generation;
use generation_atomic_u64::GenerationAtomicU64;
pub use generation_commit::GenerationCommit;
#[cfg(test)]
mod tests {
    #[test]
    fn only_latest_generation_can_commit() {
        let gate = super::GenerationGate::default();
        let first = gate.begin();
        let second = gate.begin();
        assert_eq!(gate.classify(first), super::GenerationCommit::Stale);
        assert_eq!(gate.classify(second), super::GenerationCommit::Current);
    }
}
