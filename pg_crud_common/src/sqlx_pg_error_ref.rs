#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub struct SqlxPgErrorRef<'error_lt>(&'error_lt sqlx::Error);

impl<'error_lt> SqlxPgErrorRef<'error_lt> {
    pub(crate) const fn get(self) -> &'error_lt sqlx::Error {
        self.0
    }
}
