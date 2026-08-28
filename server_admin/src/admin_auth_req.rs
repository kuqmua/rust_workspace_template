#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{AdminPeerAddr, HttpAdminHeaderMap, SharedAdminAuthSvcStateArc};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone)]
pub(crate) struct AdminAuthReq {
    pub(super) headers: HttpAdminHeaderMap,
    pub(super) state: SharedAdminAuthSvcStateArc,
    pub(super) peer: AdminPeerAddr,
}
