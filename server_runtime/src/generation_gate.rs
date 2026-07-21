#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Generation(u64);
impl From<u64> for Generation {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenerationCommit {
    Current,
    Stale,
}

#[derive(Debug, Default)]
pub struct GenerationGate {
    current: StdGenerationAtomicU64,
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

#[derive(Debug, Default)]
struct StdGenerationAtomicU64(std::sync::atomic::AtomicU64);
impl From<std::sync::atomic::AtomicU64> for StdGenerationAtomicU64 {
    fn from(value: std::sync::atomic::AtomicU64) -> Self {
        Self(value)
    }
}

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
