#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
pub struct AxumHttpStatusCode(axum::http::StatusCode);

impl AxumHttpStatusCode {
    #[must_use]
    pub fn bad_request() -> Self {
        Self::from(axum::http::StatusCode::BAD_REQUEST)
    }

    #[must_use]
    pub fn im_a_teapot() -> Self {
        Self::from(axum::http::StatusCode::IM_A_TEAPOT)
    }

    #[must_use]
    pub fn payload_too_large() -> Self {
        Self::from(axum::http::StatusCode::PAYLOAD_TOO_LARGE)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_status_constructors_preserve_exact_native_codes() {
        [
            (
                super::AxumHttpStatusCode::bad_request(),
                axum::http::StatusCode::BAD_REQUEST,
            ),
            (
                super::AxumHttpStatusCode::im_a_teapot(),
                axum::http::StatusCode::IM_A_TEAPOT,
            ),
            (
                super::AxumHttpStatusCode::payload_too_large(),
                axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            ),
        ]
        .into_iter()
        .fold((), |(), (status, expected)| {
            assert_eq!(status.get(), expected);
            assert_eq!(super::AxumHttpStatusCode::from(expected), status);
        });
    }
}
