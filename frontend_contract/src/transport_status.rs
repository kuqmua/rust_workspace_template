#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::Display,
    newtype::IntoInnerFrom,
    newtype::TryFrom,
)]
#[try_from(error = crate::HttpStatusTryFromU16Error, validator = |value: &u16| {
    if (100u16..1_000u16).contains(value) { Ok(()) } else { Err(crate::HttpStatusTryFromU16Error) }
})]
pub struct TransportStatus(u16);

impl From<crate::KnownHttpStatus> for TransportStatus {
    fn from(value: crate::KnownHttpStatus) -> Self {
        Self(value.get())
    }
}
