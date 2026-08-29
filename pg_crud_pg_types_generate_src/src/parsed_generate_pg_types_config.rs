#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct ParsedGeneratePgTypesConfig(
    pub(super) crate::generate_pg_types_config::GeneratePgTypesConfig,
);
