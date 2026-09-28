#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum GitInfoResponseError {
    #[error("{0}")]
    CommitLink(git_info::git_info_string_try_from_string_error::GitInfoStringTryFromStringError),
}
impl axum::response::IntoResponse for GitInfoResponseError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::CommitLink(_) => axum::response::IntoResponse::into_response(
                frontend_contract::api_problem_error::ApiProblemError::Internal(
                    frontend_contract::api_problem_status::ApiProblemStatus::from(
                        frontend_contract::known_http_status::KnownHttpStatus::InternalServerError,
                    ),
                ),
            ),
        }
    }
}
