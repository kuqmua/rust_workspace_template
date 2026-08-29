#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
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

pub use super::generation::Generation;
use super::generation_atomic_u64::GenerationAtomicU64;
pub use super::generation_commit::GenerationCommit;
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
