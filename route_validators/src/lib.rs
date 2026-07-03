pub mod check_body_size;
pub mod check_commit;
pub mod hdr_val;
#[cfg(test)]
mod test_hlp;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RouteValidationStatusCode {
    BadRequest,
    Ok,
    PayloadTooLarge,
    Unauthorized,
}

pub trait GetRouteValidationStatusCode {
    const ROUTE_VALIDATION_STATUS_CODE: RouteValidationStatusCode;

    #[must_use]
    fn get_route_validation_status_code(&self) -> RouteValidationStatusCode {
        Self::ROUTE_VALIDATION_STATUS_CODE
    }
}

#[must_use]
pub const fn check_body_size(
    validation: check_body_size::BodySizeValidation,
) -> check_body_size::BodySizeValidationResult {
    check_body_size::check_body_size(validation)
}

#[must_use]
pub const fn check_commit(
    validation: check_commit::CommitValidation,
) -> check_commit::CommitValidationResult {
    check_commit::check_commit(validation)
}
