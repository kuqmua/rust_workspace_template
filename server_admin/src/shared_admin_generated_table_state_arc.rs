#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    newtype::DebugRedacted,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub struct SharedAdminGeneratedTableStateArc(
    std::sync::Arc<
        dyn pg_table::combination_of_app_state_logic_traits::CombinationOfAppStateLogicTraits,
    >,
);
