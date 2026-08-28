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
#[try_from(error = crate::HttpStatusTryFromU16Error, validator = TransportStatus::validate)]
pub struct TransportStatus(u16);

impl From<crate::KnownHttpStatus> for TransportStatus {
    fn from(value: crate::KnownHttpStatus) -> Self {
        Self(value.get())
    }
}

impl TransportStatus {
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(
        clippy::single_call_fn,
        clippy::trivially_copy_pass_by_ref,
        reason = "derive-generated TryFrom owns the single validation call"
    )]
    fn validate(value: &u16) -> Result<(), crate::HttpStatusTryFromU16Error> {
        if (100u16..1_000u16).contains(value) {
            Ok(())
        } else {
            Err(crate::HttpStatusTryFromU16Error)
        }
    }
}
