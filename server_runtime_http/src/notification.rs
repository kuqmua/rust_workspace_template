#[path = "notification/axum_notification_json.rs"]
mod axum_notification_json;
#[path = "notification/axum_notification_router.rs"]
mod axum_notification_router;
#[path = "notification/axum_notification_state.rs"]
mod axum_notification_state;
#[path = "notification/http_notification_header_map.rs"]
mod http_notification_header_map;
#[path = "notification/notification_api_token.rs"]
mod notification_api_token;
#[path = "notification/notification_api_token_authorized.rs"]
mod notification_api_token_authorized;
#[path = "notification/notification_api_token_error.rs"]
mod notification_api_token_error;
#[path = "notification/notification_api_token_ref.rs"]
mod notification_api_token_ref;
#[path = "notification/notification_message.rs"]
mod notification_message;
#[path = "notification/notification_message_error.rs"]
mod notification_message_error;
#[path = "notification/notification_request.rs"]
mod notification_request;
#[path = "notification/notification_router.rs"]
mod notification_router;
#[path = "notification/notification_sender.rs"]
mod notification_sender;
#[path = "notification/notification_service_state.rs"]
mod notification_service_state;
#[path = "notification/send_notification.rs"]
mod send_notification;

use axum_notification_json::AxumNotificationJson;
pub use axum_notification_router::AxumNotificationRouter;
use axum_notification_state::AxumNotificationState;
use http_notification_header_map::HttpNotificationHeaderMap;
pub use notification_api_token::NotificationApiToken;
pub use notification_api_token_authorized::NotificationApiTokenAuthorized;
pub use notification_api_token_error::NotificationApiTokenError;
pub use notification_api_token_ref::NotificationApiTokenRef;
pub use notification_message::NotificationMessage;
pub use notification_message_error::NotificationMessageError;
pub use notification_request::NotificationRequest;
pub use notification_router::notification_router;
pub use notification_sender::NotificationSender;
pub use notification_service_state::NotificationServiceState;
use send_notification::send_notification;

#[cfg(test)]
mod tests {
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
    struct TestSender;
    impl super::NotificationSender for TestSender {
        type Error = std::convert::Infallible;
        fn send(
            &self,
            _message: super::NotificationMessage,
        ) -> impl Future<Output = Result<(), Self::Error>> + Send {
            std::future::ready(Ok(()))
        }
    }

    #[test]
    fn api_token_debug_is_redacted() {
        let token = super::NotificationApiToken::try_from(String::from(
            constants_str::TEST_NOTIFICATION_API_TOKEN,
        ))
        .expect("9ac320d1 api_token_debug_is_redacted invariant must hold");
        assert!(!format!("{token:?}").contains(constants_str::TEST_NOTIFICATION_API_TOKEN));
        assert!(
            !format!(
                "{:?}",
                super::NotificationApiTokenRef::from(constants_str::TEST_NOTIFICATION_API_TOKEN)
            )
            .contains(constants_str::TEST_NOTIFICATION_API_TOKEN)
        );
        assert!(bool::from(token.authorizes(
            super::NotificationApiTokenRef::from(constants_str::TEST_NOTIFICATION_API_TOKEN,)
        )));
    }

    #[test]
    fn message_deserialization_uses_length_validation() {
        let json = serde_json::Value::String(constants_str::X.repeat(65_537usize)).to_string();
        let Err(_error) = serde_json::from_str::<super::NotificationMessage>(&json) else {
            panic!("ecef8003");
        };
    }

    #[tokio::test]
    async fn router_requires_token_and_delivers_valid_request() {
        let token = super::NotificationApiToken::try_from(
            constants_str::TEST_NOTIFICATION_API_TOKEN.to_owned(),
        )
        .expect("cd592f18 router_requires_token_and_delivers_valid_request invariant must hold");
        let router: axum::Router =
            super::notification_router(super::NotificationServiceState::new(
                token,
                TestSender,
                std::num::NonZeroUsize::MIN.into(),
            ))
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
