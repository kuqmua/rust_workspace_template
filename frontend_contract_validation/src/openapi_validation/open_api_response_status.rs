#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, newtype::TryFrom,
)]
#[try_from(
    error = frontend_contract::domain_types::HttpStatusTryFromU16Error,
    validator = OpenApiResponseStatus::validate
)]
pub struct OpenApiResponseStatus(pub(super) u16);
impl OpenApiResponseStatus {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
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
