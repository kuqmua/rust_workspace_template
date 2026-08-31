#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct StdCookieMaxAgeSeconds(u64);

impl StdCookieMaxAgeSeconds {
    pub(crate) const fn get(self) -> u64 {
        self.0
    }
}
