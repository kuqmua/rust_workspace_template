#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct SqlxPgRateLimitPoolRef<'value_lt>(pub(super) &'value_lt sqlx::PgPool);
