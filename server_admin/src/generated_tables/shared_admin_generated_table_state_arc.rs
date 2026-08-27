#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::DebugRedacted, newtype::FromInner,
)]
pub struct SharedAdminGeneratedTableStateArc(
    pub(super) std::sync::Arc<dyn pg_table::domain_types::CombinationOfAppStateLogicTraits>,
);
