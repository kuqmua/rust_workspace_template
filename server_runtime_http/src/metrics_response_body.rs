#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_into_inner::IntoInner,
)]
pub struct MetricsResponseBody(String);

impl axum::response::IntoResponse for MetricsResponseBody {
    fn into_response(self) -> axum::response::Response {
        axum::response::IntoResponse::into_response(self.0)
    }
}

impl TryFrom<String> for MetricsResponseBody {
    type Error = crate::metrics_response_body_error::MetricsResponseBodyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= constants_usize::VALUE_8_388_608 {
            Ok(Self(value))
        } else {
            Err(crate::metrics_response_body_error::MetricsResponseBodyError::TooLarge)
        }
    }
}
