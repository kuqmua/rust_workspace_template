#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(super) enum AdminAssetsError {
    #[error("administrator asset read failed: {0}")]
    Read(to_err_string::error_text::ErrorText),
}

impl axum::response::IntoResponse for AdminAssetsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Read(_error) => axum::response::IntoResponse::into_response(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_admin_asset_read_error_returns_internal_server_error() {
        let error_text = to_err_string::error_text::ErrorText::try_from(String::from(
            constants_str::SECRET_VALUE,
        ))
        .unwrap_or_else(to_err_string::error_text::ErrorText::from);
        let response =
            axum::response::IntoResponse::into_response(super::AdminAssetsError::Read(error_text));
        assert_eq!(
            response.status(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        );
        assert!(response.headers().is_empty());
        assert_eq!(
            axum::body::HttpBody::size_hint(response.body()).exact(),
            Some(0u64)
        );
    }
}
