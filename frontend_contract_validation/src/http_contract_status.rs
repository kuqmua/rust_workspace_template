#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, newtype::TryFrom,
)]
#[try_from(error = frontend_contract::domain_types::HttpStatusTryFromU16Error, validator = HttpContractStatus::validate)]
pub struct HttpContractStatus(u16);

impl HttpContractStatus {
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(
        clippy::single_call_fn,
        clippy::trivially_copy_pass_by_ref,
        reason = "derive-generated TryFrom owns the single validation call"
    )]
    fn validate(
        value: &u16,
    ) -> Result<(), frontend_contract::domain_types::HttpStatusTryFromU16Error> {
        if (100u16..1_000u16).contains(value) {
            Ok(())
        } else {
            Err(frontend_contract::domain_types::HttpStatusTryFromU16Error)
        }
    }
}
