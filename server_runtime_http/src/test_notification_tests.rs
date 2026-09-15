#[cfg(test)]
mod tests {
    #[test]
    fn test_api_token_debug_is_redacted() {
        let token = crate::notification_api_token::NotificationApiToken::try_from(String::from(
            constants_str::TEST_NOTIFICATION_API_TOKEN,
        ))
        .expect(constants_str::DIAGNOSTIC_9AC320D1);
        assert!(!format!("{token:?}").contains(constants_str::TEST_NOTIFICATION_API_TOKEN));
        assert!(
            !format!(
                "{:?}",
                crate::notification_api_token_ref::NotificationApiTokenRef::from(
                    constants_str::TEST_NOTIFICATION_API_TOKEN
                )
            )
            .contains(constants_str::TEST_NOTIFICATION_API_TOKEN)
        );
        assert!(bool::from(token.authorizes(
            crate::notification_api_token_ref::NotificationApiTokenRef::from(
                constants_str::TEST_NOTIFICATION_API_TOKEN,
            )
        )));
    }

    #[test]
    fn test_message_deserialization_uses_length_validation() {
        let json = serde_json::Value::String(constants_str::X.repeat(65_537usize)).to_string();
        let Err(_error) = serde_json::from_str::<
            crate::runtime_notification_message::RuntimeNotificationMessage,
        >(&json) else {
            std::panic::panic_any(constants_str::PANIC_ECEF8003);
        };
    }
}
