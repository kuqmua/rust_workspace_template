#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct PgRateLimitQueryRef(&'static str);

impl PgRateLimitQueryRef {
    pub(crate) const fn get(self) -> &'static str {
        self.0
    }
}
