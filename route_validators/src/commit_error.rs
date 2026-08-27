#[derive(
    Debug, thiserror::Error, location::Location, optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum CommitError {
    CommitNotEq {
        #[eo_to_err_string]
        commit_not_eq: super::CommitNotEqMessage,
        #[eo_to_err_string]
        commit_to_use: super::CommitToUse,
        location: location_lib::domain_types::Location,
    },
    CommitToStrConversion {
        location: location_lib::domain_types::Location,
        #[eo_to_err_string]
        commit_to_str_conversion: super::AxumCommitToStrConversionError,
    },
    NoCommitHeader {
        #[eo_to_err_string]
        no_commit_header: super::NoCommitHeaderMessage,
        location: location_lib::domain_types::Location,
    },
}

impl crate::domain_types::AxumHttpStatusCodeProvider for CommitError {
    fn axum_http_status_code(&self) -> crate::domain_types::AxumHttpStatusCode {
        crate::domain_types::AxumHttpStatusCode::bad_request()
    }
}

impl CommitError {
    #[allow(clippy::single_call_fn)] // keeps mismatch error construction reusable and explicit
    pub(super) fn commit_not_eq(commit_to_use: super::CommitToUse) -> Self {
        Self::CommitNotEq {
            commit_not_eq: super::CommitNotEqMessage::from(
                constants_str::ROUTE_VALIDATORS_COMMIT_NOT_EQ_MSG,
            ),
            commit_to_use,
            location: location_macros::location!(),
        }
    }

    #[allow(clippy::single_call_fn)] // keeps header to-str conversion error construction reusable
    pub(super) fn commit_to_str_conversion(
        commit_to_str_conversion: super::AxumCommitToStrConversionError,
    ) -> Self {
        Self::CommitToStrConversion {
            commit_to_str_conversion,
            location: location_macros::location!(),
        }
    }

    #[allow(clippy::single_call_fn)] // keeps missing-commit-header error construction reusable
    pub(super) fn no_commit_header() -> Self {
        Self::NoCommitHeader {
            no_commit_header: super::NoCommitHeaderMessage::from(
                constants_str::ROUTE_VALIDATORS_NO_COMMIT_HEADER_MSG,
            ),
            location: location_macros::location!(),
        }
    }
}
