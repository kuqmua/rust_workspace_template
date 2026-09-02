#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_new::New)]
#[constructor(pub(super))]
pub(crate) struct GeneratedCrateStep {
    #[getters(copy)]
    args: &'static [&'static str],
    #[getters(copy)]
    phase: crate::generated_crate_phase::GeneratedCratePhase,
}
