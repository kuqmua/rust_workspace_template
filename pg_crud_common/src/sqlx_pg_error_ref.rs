#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct SqlxPgErrorRef<'error_lt>(&'error_lt sqlx::Error);

impl<'error_lt> SqlxPgErrorRef<'error_lt> {
    pub(crate) const fn get(self) -> &'error_lt sqlx::Error {
        self.0
    }
}
