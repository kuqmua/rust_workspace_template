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
#[try_from(error = super::super::HttpStatusTryFromU16Error, validator = TransportStatus::validate)]
pub struct TransportStatus(u16);

impl From<super::super::KnownHttpStatus> for TransportStatus {
    fn from(value: super::super::KnownHttpStatus) -> Self {
        Self(value.get())
    }
}

impl TransportStatus {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)]
    fn validate(value: &u16) -> Result<(), super::super::HttpStatusTryFromU16Error> {
        if (100u16..1_000u16).contains(value) {
            Ok(())
        } else {
            Err(super::super::HttpStatusTryFromU16Error)
        }
    }
}
