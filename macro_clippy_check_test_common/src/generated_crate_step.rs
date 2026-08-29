#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_constructor::New)]
#[constructor(pub(super))]
pub(crate) struct GeneratedCrateStep {
    args: &'static [&'static str],
    phase: crate::generated_crate_phase::GeneratedCratePhase,
}

impl GeneratedCrateStep {
    pub(crate) const fn args(&self) -> &'static [&'static str] {
        self.args
    }

    pub(crate) const fn phase(&self) -> crate::generated_crate_phase::GeneratedCratePhase {
        self.phase
    }
}
