#![allow(
    unused_variables,
    reason = "test notification trait fixtures preserve repository type-based parameter names"
)]

#[cfg(test)]
mod tests {
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
    struct TestSender;
    impl crate::notification_sender::NotificationSender for TestSender {
        type Error = std::convert::Infallible;
        fn send(
            &self,
            runtime_notification_message: crate::runtime_notification_message::RuntimeNotificationMessage,
        ) -> impl Future<Output = Result<(), Self::Error>> + Send {
            std::future::ready(Ok(()))
        }
    }

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

    #[tokio::test]
    async fn test_router_requires_token_and_delivers_valid_request() {
        let token = crate::notification_api_token::NotificationApiToken::try_from(
            constants_str::TEST_NOTIFICATION_API_TOKEN.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_CD592F18);
        let router: axum::Router = crate::notification_router::notification_router(
            crate::notification_service_state::NotificationServiceState::new(
                token,
                TestSender,
                std::num::NonZeroUsize::MIN.into(),
            ),
        )
        .into();
        let request = http::Request::builder()
            .method(http::Method::POST)
            .uri(constants_str::NOTIFICATIONS_PATH)
            .header(
                http::header::AUTHORIZATION,
                constants_str::TEST_BEARER_AUTHORIZATION,
            )
            .header(http::header::CONTENT_TYPE, constants_str::APPLICATION_JSON)
            .body(axum::body::Body::from(
                constants_str::TEST_NOTIFICATION_REQUEST_JSON,
            ))
            .expect(constants_str::DIAGNOSTIC_9E3B810C);
        let response = tower::ServiceExt::oneshot(router, request)
            .await
            .expect(constants_str::DIAGNOSTIC_DB062FE4);
        assert_eq!(response.status(), http::StatusCode::NO_CONTENT);
    }
}

// Root-owned module compatibility wrappers.
