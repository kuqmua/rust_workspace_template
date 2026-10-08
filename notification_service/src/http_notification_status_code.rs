#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct HttpNotificationStatusCode(http::StatusCode);
impl axum::response::IntoResponse for HttpNotificationStatusCode {
    fn into_response(self) -> axum::response::Response {
        axum::response::IntoResponse::into_response(self.0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_notification_status_adapter_preserves_standard_and_extension_statuses() {
        assert!((100u16..=999u16).all(|value| {
            http::StatusCode::from_u16(value).is_ok_and(|status_code| {
                axum::response::IntoResponse::into_response(
                    crate::http_notification_status_code::HttpNotificationStatusCode::from(
                        status_code,
                    ),
                )
                .status()
                    == status_code
            })
        }));
    }
}
