#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BodySizeValidation {
    BodyEqualsLimit,
    BodyExceedsLimit,
    BodyFitsLimit,
    EmptyBodyWithZeroLimit,
    NonEmptyBodyWithZeroLimit,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BodySizeValidationResult {
    Accepted,
    ReachedMaximumSizeOfBody,
}

impl crate::GetRouteValidationStatusCode for BodySizeValidationResult {
    const ROUTE_VALIDATION_STATUS_CODE: crate::RouteValidationStatusCode =
        crate::RouteValidationStatusCode::PayloadTooLarge;
}

#[must_use]
pub const fn check_body_size(validation: BodySizeValidation) -> BodySizeValidationResult {
    match validation {
        BodySizeValidation::BodyEqualsLimit
        | BodySizeValidation::BodyFitsLimit
        | BodySizeValidation::EmptyBodyWithZeroLimit => BodySizeValidationResult::Accepted,
        BodySizeValidation::BodyExceedsLimit | BodySizeValidation::NonEmptyBodyWithZeroLimit => {
            BodySizeValidationResult::ReachedMaximumSizeOfBody
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_body_size_accepts_body_that_fits_limit() -> Result<(), String> {
        let result = crate::check_body_size::check_body_size(
            crate::check_body_size::BodySizeValidation::BodyFitsLimit,
        );
        if result == crate::check_body_size::BodySizeValidationResult::Accepted {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn check_body_size_accepts_body_equal_to_limit() -> Result<(), String> {
        let result = crate::check_body_size::check_body_size(
            crate::check_body_size::BodySizeValidation::BodyEqualsLimit,
        );
        if result == crate::check_body_size::BodySizeValidationResult::Accepted {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn check_body_size_accepts_empty_body_with_zero_limit() -> Result<(), String> {
        let result = crate::check_body_size::check_body_size(
            crate::check_body_size::BodySizeValidation::EmptyBodyWithZeroLimit,
        );
        if result == crate::check_body_size::BodySizeValidationResult::Accepted {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn check_body_size_rejects_body_that_exceeds_limit() -> Result<(), String> {
        let result = crate::check_body_size::check_body_size(
            crate::check_body_size::BodySizeValidation::BodyExceedsLimit,
        );
        if result == crate::check_body_size::BodySizeValidationResult::ReachedMaximumSizeOfBody {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn check_body_size_rejects_non_empty_body_with_zero_limit() -> Result<(), String> {
        let result = crate::check_body_size::check_body_size(
            crate::check_body_size::BodySizeValidation::NonEmptyBodyWithZeroLimit,
        );
        if result == crate::check_body_size::BodySizeValidationResult::ReachedMaximumSizeOfBody {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }
}
