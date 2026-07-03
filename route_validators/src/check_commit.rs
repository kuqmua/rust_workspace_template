#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CommitValidation {
    Disabled,
    Enabled(CommitHeaderValidation),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CommitHeaderValidation {
    DifferentProjectCommit,
    Missing,
    ProjectCommit,
    ValueNotText,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CommitValidationResult {
    Accepted,
    CommitNotEqual,
    CommitToTextConversionFailed,
    NoCommitHeader,
}

impl crate::GetRouteValidationStatusCode for CommitValidationResult {
    const ROUTE_VALIDATION_STATUS_CODE: crate::RouteValidationStatusCode =
        crate::RouteValidationStatusCode::BadRequest;
}

#[must_use]
pub const fn check_commit(validation: CommitValidation) -> CommitValidationResult {
    match validation {
        CommitValidation::Disabled => CommitValidationResult::Accepted,
        CommitValidation::Enabled(CommitHeaderValidation::ProjectCommit) => {
            CommitValidationResult::Accepted
        }
        CommitValidation::Enabled(CommitHeaderValidation::DifferentProjectCommit) => {
            CommitValidationResult::CommitNotEqual
        }
        CommitValidation::Enabled(CommitHeaderValidation::Missing) => {
            CommitValidationResult::NoCommitHeader
        }
        CommitValidation::Enabled(CommitHeaderValidation::ValueNotText) => {
            CommitValidationResult::CommitToTextConversionFailed
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_commit_is_skipped_when_validation_is_disabled() -> Result<(), String> {
        let result =
            crate::check_commit::check_commit(crate::check_commit::CommitValidation::Disabled);
        if result == crate::check_commit::CommitValidationResult::Accepted {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn check_commit_accepts_project_commit_when_enabled() -> Result<(), String> {
        let result =
            crate::check_commit::check_commit(crate::check_commit::CommitValidation::Enabled(
                crate::check_commit::CommitHeaderValidation::ProjectCommit,
            ));
        if result == crate::check_commit::CommitValidationResult::Accepted {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn check_commit_rejects_wrong_commit_when_enabled() -> Result<(), String> {
        let result =
            crate::check_commit::check_commit(crate::check_commit::CommitValidation::Enabled(
                crate::check_commit::CommitHeaderValidation::DifferentProjectCommit,
            ));
        if result == crate::check_commit::CommitValidationResult::CommitNotEqual {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn check_commit_rejects_missing_commit_header_when_enabled() -> Result<(), String> {
        let result =
            crate::check_commit::check_commit(crate::check_commit::CommitValidation::Enabled(
                crate::check_commit::CommitHeaderValidation::Missing,
            ));
        if result == crate::check_commit::CommitValidationResult::NoCommitHeader {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn check_commit_rejects_non_text_commit_header_when_enabled() -> Result<(), String> {
        let result =
            crate::check_commit::check_commit(crate::check_commit::CommitValidation::Enabled(
                crate::check_commit::CommitHeaderValidation::ValueNotText,
            ));
        if result == crate::check_commit::CommitValidationResult::CommitToTextConversionFailed {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }
}
