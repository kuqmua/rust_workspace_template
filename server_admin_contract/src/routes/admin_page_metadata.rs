#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{AdminPageClientMode, AdminPageNavigation};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdminPageMetadata {
    pub(super) client_mode: AdminPageClientMode,
    pub(super) navigation: Option<AdminPageNavigation>,
}
impl AdminPageMetadata {
    pub(super) const fn new(
        client_mode: AdminPageClientMode,
        navigation: Option<AdminPageNavigation>,
    ) -> Self {
        Self {
            client_mode,
            navigation,
        }
    }
}
