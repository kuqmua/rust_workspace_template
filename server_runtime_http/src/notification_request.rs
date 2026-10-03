#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    proc_macro_new::New,
)]
#[serde(deny_unknown_fields)]
pub struct NotificationRequest {
    message: crate::runtime_notification_message::RuntimeNotificationMessage,
}
impl NotificationRequest {
    pub(crate) fn into_message(
        self,
    ) -> crate::runtime_notification_message::RuntimeNotificationMessage {
        self.message
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_notification_request_round_trip_rejects_missing_unknown_and_invalid_message_fields() {
        assert!(
            serde_json::from_str::<serde_json::Value>(
                constants_str::TEST_NOTIFICATION_REQUEST_JSON
            )
            .is_ok_and(|value| {
                serde_json::from_value::<crate::notification_request::NotificationRequest>(
                    value.clone(),
                )
                .is_ok_and(|request| {
                    serde_json::to_value(request).is_ok_and(|serialized| serialized == value)
                })
            })
        );
        assert!(
            [
                serde_json::json!({}),
                serde_json::json!({(stringify!(message)): constants_str::PG_CRUD_EMPTY_SQL_SUFFIX}),
                serde_json::json!({(stringify!(message)): null}),
                serde_json::json!({(stringify!(message)): true}),
                serde_json::json!({(stringify!(message)): 1u8}),
                serde_json::json!({(stringify!(message)): []}),
                serde_json::json!({(stringify!(message)): {}}),
                serde_json::json!({(stringify!(message)): constants_str::X, (constants_str::X): constants_str::X}),
            ]
            .into_iter()
            .all(|value| {
                serde_json::from_value::<crate::notification_request::NotificationRequest>(value)
                    .is_err_and(|error| error.is_data())
            })
        );
    }

    #[test]
    fn test_notification_request_rejects_duplicate_message_field() {
        let text = format!(
            "{{\"{0}\":\"{1}\",\"{0}\":\"{1}\"}}",
            stringify!(message),
            constants_str::X
        );
        assert!(
            serde_json::from_str::<crate::notification_request::NotificationRequest>(&text)
                .is_err_and(|error| error.is_data())
        );
    }
}
