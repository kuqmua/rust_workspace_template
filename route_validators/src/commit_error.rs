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

impl crate::AxumHttpStatusCodeProvider for CommitError {
    fn axum_http_status_code(&self) -> crate::AxumHttpStatusCode {
        crate::AxumHttpStatusCode::bad_request()
    }
}
