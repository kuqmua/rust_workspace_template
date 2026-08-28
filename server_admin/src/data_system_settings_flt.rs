#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::domain_types::generated_tables::StdOptionalOptionalAdminSystemSettingsWhereMany;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub(crate) struct DataSystemSettingsFlt(pub(crate) StdOptionalOptionalAdminSystemSettingsWhereMany);
