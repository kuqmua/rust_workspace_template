#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_into_inner::IntoInner,
)]
pub struct MetricsResponseBody(
    bounded_types::bounded_string::BoundedString<0usize, 8_388_608usize, false>,
);

impl axum::response::IntoResponse for MetricsResponseBody {
    fn into_response(self) -> axum::response::Response {
        axum::response::IntoResponse::into_response(self.0.into_string())
    }
}

impl TryFrom<String> for MetricsResponseBody {
    type Error = crate::metrics_response_body_error::MetricsResponseBodyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= constants_usize::VALUE_8_388_608 {
            bounded_types::bounded_string::BoundedString::try_from(value)
                .map(Self)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        ..
                    }
                    | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                        ..
                    } => Self::Error::TooLarge,
                })
        } else {
            Err(crate::metrics_response_body_error::MetricsResponseBodyError::TooLarge)
        }
    }
}
