#[path = "generation_gate/generation.rs"]
mod generation;
#[path = "generation_gate/generation_atomic_u64.rs"]
mod generation_atomic_u64;
#[path = "generation_gate/generation_commit.rs"]
mod generation_commit;
#[path = "generation_gate/generation_gate.rs"]
mod generation_gate;

pub use generation::Generation;
use generation_atomic_u64::GenerationAtomicU64;
pub use generation_commit::GenerationCommit;
pub use generation_gate::GenerationGate;

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
