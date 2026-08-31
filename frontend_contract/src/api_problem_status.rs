#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::IntoInnerFrom,
    newtype::FromGetter,
    newtype::TryFrom,
)]
#[from_getter(source = crate::known_http_status::KnownHttpStatus, getter = get)]
#[serde(try_from = "u16")]
#[try_from(error = crate::http_status_try_from_u16_error::HttpStatusTryFromU16Error, validator = |value: &u16| {
    if (100u16..1_000u16).contains(value) { Ok(()) } else { Err(crate::http_status_try_from_u16_error::HttpStatusTryFromU16Error::OutOfRange) }
})]
pub struct ApiProblemStatus(u16);
