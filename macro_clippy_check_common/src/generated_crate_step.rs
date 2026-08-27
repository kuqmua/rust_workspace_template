#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct GeneratedCrateStep {
    args: &'static [&'static str],
    phase: crate::generated_crate_steps::GeneratedCratePhase,
}

impl GeneratedCrateStep {
    pub(crate) const fn args(&self) -> &'static [&'static str] {
        self.args
    }

    pub(super) const fn new(
        args: &'static [&'static str],
        phase: crate::generated_crate_steps::GeneratedCratePhase,
    ) -> Self {
        Self { args, phase }
    }

    pub(crate) const fn phase(&self) -> crate::generated_crate_steps::GeneratedCratePhase {
        self.phase
    }
}
