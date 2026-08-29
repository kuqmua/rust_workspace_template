pub(crate) use super::admin_badge::AdminBadge;
pub(crate) use super::admin_badge_variant::AdminBadgeVariant;
// Root-owned module compatibility wrappers.
pub(crate) mod admin_badge {
    pub use super::super::admin_badge::*;
}
pub(crate) mod admin_badge_variant {
    pub use super::super::admin_badge_variant::*;
}
