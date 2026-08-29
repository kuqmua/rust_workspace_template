#[derive(
    Debug, thiserror::Error, location::Location, optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum CommitError {
    CommitNotEq {
        #[eo_to_err_string]
        commit_not_eq: crate::commit_not_eq_message::CommitNotEqMessage,
        #[eo_to_err_string]
        commit_to_use: crate::commit_to_use::CommitToUse,
        location: location_lib::location::Location,
    },
    CommitToStrConversion {
        location: location_lib::location::Location,
        #[eo_to_err_string]
        commit_to_str_conversion:
            crate::axum_commit_to_str_conversion_error::AxumCommitToStrConversionError,
    },
    NoCommitHeader {
        #[eo_to_err_string]
        no_commit_header: crate::no_commit_header_message::NoCommitHeaderMessage,
        location: location_lib::location::Location,
    },
}

impl crate::axum_http_status_code_provider::AxumHttpStatusCodeProvider for CommitError {
    fn axum_http_status_code(&self) -> crate::axum_http_status_code::AxumHttpStatusCode {
        crate::axum_http_status_code::AxumHttpStatusCode::bad_request()
    }
}
