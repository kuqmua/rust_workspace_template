#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct HttpErrorStatus(http::StatusCode);

impl HttpErrorStatus {
    pub(crate) const fn as_u16(self) -> u16 {
        self.0.as_u16()
    }
}
