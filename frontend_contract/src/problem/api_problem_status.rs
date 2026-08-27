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
#[try_from(error = crate::domain_types::HttpStatusTryFromU16Error, validator = ApiProblemStatus::validate)]
pub struct ApiProblemStatus(u16);

impl From<crate::domain_types::KnownHttpStatus> for ApiProblemStatus {
    fn from(value: crate::domain_types::KnownHttpStatus) -> Self {
        Self(value.get())
    }
}

impl ApiProblemStatus {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)]
    fn validate(value: &u16) -> Result<(), crate::domain_types::HttpStatusTryFromU16Error> {
        if (100u16..1_000u16).contains(value) {
            Ok(())
        } else {
            Err(crate::domain_types::HttpStatusTryFromU16Error)
        }
    }
}
