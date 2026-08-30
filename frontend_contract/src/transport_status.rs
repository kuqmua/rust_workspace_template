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
#[try_from(error = crate::http_status_try_from_u16_error::HttpStatusTryFromU16Error, validator = |value: &u16| {
    if (100u16..1_000u16).contains(value) { Ok(()) } else { Err(crate::http_status_try_from_u16_error::HttpStatusTryFromU16Error::OutOfRange) }
})]
pub struct TransportStatus(u16);

impl From<crate::known_http_status::KnownHttpStatus> for TransportStatus {
    fn from(value: crate::known_http_status::KnownHttpStatus) -> Self {
        Self(value.get())
    }
}
