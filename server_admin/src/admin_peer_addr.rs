#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct AdminPeerAddr(pub(crate) crate::AdminSocketAddr);
impl AdminPeerAddr {
    pub(crate) const fn socket_addr(self) -> crate::AdminSocketAddr {
        self.0
    }
}
