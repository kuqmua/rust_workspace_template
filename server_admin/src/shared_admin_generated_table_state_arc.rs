#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    proc_macro_newtype_debug_redacted::DebugRedacted,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
pub struct SharedAdminGeneratedTableStateArc(
    std::sync::Arc<
        dyn pg_table::combination_of_app_state_logic_traits::CombinationOfAppStateLogicTraits,
    >,
);
