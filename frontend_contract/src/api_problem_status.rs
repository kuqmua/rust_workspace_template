#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_newtype_from_getter::FromGetter,
)]
#[from_getter(source = crate::known_http_status::KnownHttpStatus, getter = get)]
#[serde(try_from = "u16")]
pub struct ApiProblemStatus(u16);
impl TryFrom<u16> for ApiProblemStatus {
    type Error = crate::http_status_try_from_u16_error::HttpStatusTryFromU16Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (100u16..1_000u16).contains(&value) {
            Ok(Self(value))
        } else {
            Err(Self::Error::OutOfRange)
        }
    }
}
