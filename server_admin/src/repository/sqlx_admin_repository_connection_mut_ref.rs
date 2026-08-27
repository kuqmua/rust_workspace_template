// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::field_scoped_visibility_modifiers)] // split wrapper representation is consumed only by its parent repository facade
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub(crate) struct SqlxAdminRepositoryConnectionMutRef<'connection_lt>(
    pub(super) &'connection_lt mut sqlx::PgConnection,
);
