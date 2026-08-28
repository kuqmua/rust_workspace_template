use crate::axum_notification_json::AxumNotificationJson;
pub use crate::axum_notification_router::AxumNotificationRouter;
use crate::axum_notification_state::AxumNotificationState;
use crate::http_notification_header_map::HttpNotificationHeaderMap;
pub use crate::notification_api_token::NotificationApiToken;
pub use crate::notification_api_token_authorized::NotificationApiTokenAuthorized;
pub use crate::notification_api_token_error::NotificationApiTokenError;
pub use crate::notification_api_token_ref::NotificationApiTokenRef;
pub use crate::notification_message::NotificationMessage;
pub use crate::notification_message_error::NotificationMessageError;
pub use crate::notification_request::NotificationRequest;
pub use crate::notification_router::notification_router;
pub use crate::notification_sender::NotificationSender;
pub use crate::notification_service_state::NotificationServiceState;
use crate::send_notification::send_notification;

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

// Root-owned module compatibility wrappers.
mod axum_notification_json {
    pub use crate::axum_notification_json::*;
}
mod axum_notification_router {
    pub use crate::axum_notification_router::*;
}
mod axum_notification_state {
    pub use crate::axum_notification_state::*;
}
mod http_notification_header_map {
    pub use crate::http_notification_header_map::*;
}
mod notification_api_token {
    pub use crate::notification_api_token::*;
}
mod notification_api_token_authorized {
    pub use crate::notification_api_token_authorized::*;
}
mod notification_api_token_error {
    pub use crate::notification_api_token_error::*;
}
mod notification_api_token_ref {
    pub use crate::notification_api_token_ref::*;
}
mod notification_message {
    pub use crate::notification_message::*;
}
mod notification_message_error {
    pub use crate::notification_message_error::*;
}
mod notification_request {
    pub use crate::notification_request::*;
}
mod notification_router {
    pub use crate::notification_router::*;
}
mod notification_sender {
    pub use crate::notification_sender::*;
}
mod notification_service_state {
    pub use crate::notification_service_state::*;
}
mod send_notification {
    pub use crate::send_notification::*;
}
