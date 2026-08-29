#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::DebugRedacted, newtype::FromInner,
)]
pub struct SharedAdminGeneratedTableStateArc(
    pub(crate)  std::sync::Arc<
        dyn pg_table::combination_of_app_state_logic_traits::CombinationOfAppStateLogicTraits,
    >,
);
