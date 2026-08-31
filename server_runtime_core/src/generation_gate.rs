#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
pub struct GenerationGate {
    current: crate::generation_atomic_u64::GenerationAtomicU64,
}

impl GenerationGate {
    #[must_use]
    pub fn begin(&self) -> crate::generation::Generation {
        crate::generation::Generation::from(
            self.current
                .fetch_add(1u64, std::sync::atomic::Ordering::AcqRel)
                .saturating_add(1u64),
        )
    }

    #[must_use]
    pub fn classify(
        &self,
        generation: crate::generation::Generation,
    ) -> crate::generation_commit::GenerationCommit {
        if self.current.load(std::sync::atomic::Ordering::Acquire) == *generation {
            crate::generation_commit::GenerationCommit::Current
        } else {
            crate::generation_commit::GenerationCommit::Stale
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn only_latest_generation_can_commit() {
        let gate = super::GenerationGate::default();
        let first = gate.begin();
        let second = gate.begin();
        assert_eq!(
            gate.classify(first),
            crate::generation_commit::GenerationCommit::Stale
        );
        assert_eq!(
            gate.classify(second),
            crate::generation_commit::GenerationCommit::Current
        );
    }
}
