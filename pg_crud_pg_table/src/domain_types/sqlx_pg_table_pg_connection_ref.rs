#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsMut, newtype::FromInner)]
pub struct SqlxPgTablePgConnectionRef<'connection_lt>(
    pub(super) &'connection_lt mut sqlx::PgConnection,
);
