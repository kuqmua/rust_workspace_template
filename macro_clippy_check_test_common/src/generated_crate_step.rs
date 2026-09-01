#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_constructor::New)]
#[constructor(pub(super))]
pub(crate) struct GeneratedCrateStep {
    #[getters(copy)]
    args: &'static [&'static str],
    #[getters(copy)]
    phase: crate::generated_crate_phase::GeneratedCratePhase,
}
