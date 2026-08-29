#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("administrator access token operation failed: {0:?}")]
#[derive(newtype::FromInner)]
pub struct AdminAccessTokenError(
    pub(crate) crate::jsonwebtoken_admin_error::JsonwebtokenAdminError,
);
