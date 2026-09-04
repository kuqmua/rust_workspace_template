#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_from_getter::FromGetter,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
#[from_getter(source = crate::known_http_status::KnownHttpStatus, getter = get)]
pub struct TransportStatus(u16);
impl TryFrom<u16> for TransportStatus {
    type Error = crate::http_status_try_from_u16_error::HttpStatusTryFromU16Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            100u16..=999u16 => Ok(Self(value)),
            _ => Err(Self::Error::OutOfRange),
        }
    }
}
