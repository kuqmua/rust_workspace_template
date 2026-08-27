#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::IntoInner,
)]
pub struct MetricsResponseBody(String);

impl axum::response::IntoResponse for MetricsResponseBody {
    fn into_response(self) -> axum::response::Response {
        axum::response::IntoResponse::into_response(self.0)
    }
}

impl TryFrom<String> for MetricsResponseBody {
    type Error = super::MetricsResponseBodyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > super::METRICS_RESPONSE_BODY_MAXIMUM_BYTES {
            Err(super::MetricsResponseBodyError)
        } else {
            Ok(Self(value))
        }
    }
}
