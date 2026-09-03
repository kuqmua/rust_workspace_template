#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
pub struct GenerationGate {
    current: crate::generation_atomic_u64::GenerationAtomicU64,
}

impl GenerationGate {
    pub fn begin(
        &self,
    ) -> Result<crate::generation::Generation, crate::generation_begin_error::GenerationBeginError>
    {
        self.current
            .try_update(
                std::sync::atomic::Ordering::AcqRel,
                std::sync::atomic::Ordering::Acquire,
                |current| current.checked_add(1u64),
            )
            .map(|previous| crate::generation::Generation::from(previous.saturating_add(1u64)))
            .map_err(|current| {
                crate::generation_begin_error::GenerationBeginError::Overflow(
                    crate::generation::Generation::from(current),
                )
            })
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
    fn test_only_latest_generation_can_commit() {
        let gate = super::GenerationGate::default();
        let first = gate.begin().expect(constants_str::DIAGNOSTIC_7DE09116);
        let second = gate.begin().expect(constants_str::DIAGNOSTIC_E1C98AA1);
        assert_eq!(
            gate.classify(first),
            crate::generation_commit::GenerationCommit::Stale
        );
        assert_eq!(
            gate.classify(second),
            crate::generation_commit::GenerationCommit::Current
        );
    }

    #[test]
    fn test_generation_overflow_does_not_reuse_an_identifier() {
        let gate = super::GenerationGate {
            current: crate::generation_atomic_u64::GenerationAtomicU64::from(
                std::sync::atomic::AtomicU64::new(u64::MAX),
            ),
        };
        assert_eq!(
            gate.begin(),
            Err(
                crate::generation_begin_error::GenerationBeginError::Overflow(
                    crate::generation::Generation::from(u64::MAX)
                )
            )
        );
        assert_eq!(
            gate.current.load(std::sync::atomic::Ordering::Acquire),
            u64::MAX
        );
    }
}
