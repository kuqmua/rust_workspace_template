#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub struct SqlxPgRateLimitPoolRef<'value_lt>(&'value_lt sqlx::PgPool);

impl<'value_lt> SqlxPgRateLimitPoolRef<'value_lt> {
    pub(crate) const fn get(self) -> &'value_lt sqlx::PgPool {
        self.0
    }
}
