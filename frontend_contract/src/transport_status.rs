#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype::Display,
    proc_macro_newtype::FromGetter,
    proc_macro_newtype::IntoInnerFrom,
    proc_macro_newtype::TryFrom,
)]
#[from_getter(source = crate::known_http_status::KnownHttpStatus, getter = get)]
#[try_from(error = crate::http_status_try_from_u16_error::HttpStatusTryFromU16Error, validator = |value: &u16| {
    if (100u16..1_000u16).contains(value) { Ok(()) } else { Err(crate::http_status_try_from_u16_error::HttpStatusTryFromU16Error::OutOfRange) }
})]
pub struct TransportStatus(u16);
