#[cfg(test)]
mod tests {
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
    struct TestSender;
    impl crate::notification_sender::NotificationSender for TestSender {
        type Error = std::convert::Infallible;
        fn send(
            &self,
            _message: crate::runtime_notification_message::RuntimeNotificationMessage,
        ) -> impl Future<Output = Result<(), Self::Error>> + Send {
            std::future::ready(Ok(()))
        }
    }

    #[test]
    fn test_api_token_debug_is_redacted() {
        let token = crate::notification_api_token::NotificationApiToken::try_from(String::from(
            constants_str::TEST_NOTIFICATION_API_TOKEN,
        ))
        .expect("9ac320d1 api_token_debug_is_redacted invariant must hold");
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
            panic!("ecef8003");
        };
    }

    #[tokio::test]
    async fn test_router_requires_token_and_delivers_valid_request() {
        let token = crate::notification_api_token::NotificationApiToken::try_from(
            constants_str::TEST_NOTIFICATION_API_TOKEN.to_owned(),
        )
        .expect("cd592f18 router_requires_token_and_delivers_valid_request invariant must hold");
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
            .expect(
                "9e3b810c router_requires_token_and_delivers_valid_request invariant must hold",
            );
        let response = tower::ServiceExt::oneshot(router, request).await.expect(
            "db062fe4 router_requires_token_and_delivers_valid_request invariant must hold",
        );
        assert_eq!(response.status(), http::StatusCode::NO_CONTENT);
    }
}

// Root-owned module compatibility wrappers.
