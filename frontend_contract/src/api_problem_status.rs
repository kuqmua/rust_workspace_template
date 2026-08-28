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
    newtype::TryFrom,
)]
#[serde(try_from = "u16")]
#[try_from(error = crate::HttpStatusTryFromU16Error, validator = |value: &u16| {
    if (100u16..1_000u16).contains(value) { Ok(()) } else { Err(crate::HttpStatusTryFromU16Error) }
})]
pub struct ApiProblemStatus(u16);

impl From<crate::KnownHttpStatus> for ApiProblemStatus {
    fn from(value: crate::KnownHttpStatus) -> Self {
        Self(value.get())
    }
}
