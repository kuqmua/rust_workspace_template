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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BodySizeValidation;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BodySizeValidationResult {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CommitValidation;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CommitValidationResult {
    Accepted,
    Rejected,
}

#[must_use]
pub const fn check_body_size(_validation: BodySizeValidation) -> BodySizeValidationResult {
    BodySizeValidationResult::Accepted
}

#[must_use]
pub const fn check_commit(_validation: CommitValidation) -> CommitValidationResult {
    CommitValidationResult::Accepted
}
