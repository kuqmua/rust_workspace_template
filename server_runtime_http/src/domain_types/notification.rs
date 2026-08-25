#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Eq, PartialEq)]
pub struct NotificationApiToken(String);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub struct NotificationApiTokenRef<'value_lt>(&'value_lt str);
impl std::fmt::Debug for NotificationApiTokenRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::NOTIFICATION_API_TOKEN_REDACTED)
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct NotificationApiTokenAuthorized(bool);

impl NotificationApiToken {
    #[must_use]
    pub fn authorizes(
        &self,
        candidate: NotificationApiTokenRef<'_>,
    ) -> NotificationApiTokenAuthorized {
        let maximum_len = self.0.len().max(candidate.0.len());
        let difference = (constants_usize::ZERO..maximum_len).fold(
            self.0.len() ^ candidate.0.len(),
            |acc, index| {
                acc | usize::from(
                    self.0
                        .as_bytes()
                        .get(index)
                        .copied()
                        .unwrap_or(constants_u8::ZERO)
                        ^ candidate
                            .0
                            .as_bytes()
                            .get(index)
                            .copied()
                            .unwrap_or(constants_u8::ZERO),
                )
            },
        );
        NotificationApiTokenAuthorized::from(difference == constants_usize::ZERO)
    }
}

impl std::fmt::Debug for NotificationApiToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::NOTIFICATION_API_TOKEN_REDACTED)
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum NotificationApiTokenError {
    #[error("notification API token must not be empty")]
    Empty,
    #[error("notification API token exceeds maximum length")]
    TooLong,
}

impl TryFrom<String> for NotificationApiToken {
    type Error = NotificationApiTokenError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(Self::Error::Empty)
        } else if value.len() > 4096usize {
            Err(Self::Error::TooLong)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(try_from = "String")]
pub struct NotificationMessage(String);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum NotificationMessageError {
    #[error("notification message must not be empty")]
    Empty,
    #[error("notification message exceeds maximum length")]
    TooLong,
}

impl TryFrom<String> for NotificationMessage {
    type Error = NotificationMessageError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(Self::Error::Empty)
        } else if value.len() > 65_536usize {
            Err(Self::Error::TooLong)
        } else {
            Ok(Self(value))
        }
    }
}

pub trait NotificationSender: Clone + Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;

    fn send(
        &self,
        message: NotificationMessage,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, serde::Deserialize, serde::Serialize,
)]
#[serde(deny_unknown_fields)]
pub struct NotificationRequest {
    message: NotificationMessage,
}
impl NotificationRequest {
    #[must_use]
    pub const fn new(message: NotificationMessage) -> Self {
        Self { message }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct NotificationServiceState<Sender> {
    permits: crate::domain_types::ArcTokioSemaphore,
    sender: Sender,
    token: NotificationApiToken,
}
impl<Sender> NotificationServiceState<Sender> {
    #[must_use]
    pub fn new(
        token: NotificationApiToken,
        sender: Sender,
        maximum_concurrency: crate::domain_types::SemaphorePermitCountNonZeroUsize,
    ) -> Self {
        Self {
            permits: crate::domain_types::ArcTokioSemaphore::new(maximum_concurrency),
            sender,
            token,
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub struct AxumNotificationRouter(axum::Router);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct HttpNotificationHeaderMap(http::HeaderMap);

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct AxumNotificationState<Sender> {
    headers: HttpNotificationHeaderMap,
    state: NotificationServiceState<Sender>,
}
impl<Sender> axum::extract::FromRequestParts<NotificationServiceState<Sender>>
    for AxumNotificationState<Sender>
where
    Sender: Clone + Send + Sync,
{
    type Rejection = std::convert::Infallible;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &NotificationServiceState<Sender>,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self {
            headers: HttpNotificationHeaderMap::from(parts.headers.clone()),
            state: state.clone(),
        }))
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct AxumNotificationJson(NotificationRequest);

impl<State> axum::extract::FromRequest<State> for AxumNotificationJson
where
    State: Send + Sync,
{
    type Rejection = axum::extract::rejection::JsonRejection;
    async fn from_request(
        req: axum::extract::Request,
        state: &State,
    ) -> Result<Self, Self::Rejection> {
        axum::Json::<NotificationRequest>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self::from(value))
    }
}
#[allow(clippy::single_call_fn)] // named handler keeps axum extractor boundaries domain-typed
async fn send_notification<Sender>(
    state: AxumNotificationState<Sender>,
    request: AxumNotificationJson,
) -> http::StatusCode
where
    Sender: NotificationSender,
{
    let authorization = state
        .headers
        .0
        .get(http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());
    let authorized = match crate::domain_types::resolve_bearer_authorization(
        crate::domain_types::HttpAuthorizationHeaderTextRef::from(authorization),
    ) {
        crate::domain_types::BearerAuthorizationResolution::Resolved(token) => bool::from(
            state
                .state
                .token
                .authorizes(NotificationApiTokenRef::from(token.as_ref())),
        ),
        crate::domain_types::BearerAuthorizationResolution::Invalid
        | crate::domain_types::BearerAuthorizationResolution::Missing => false,
    };
    if !authorized {
        return http::StatusCode::UNAUTHORIZED;
    }
    let Some(_permit) = state.state.permits.try_acquire() else {
        return http::StatusCode::TOO_MANY_REQUESTS;
    };
    match state.state.sender.send(request.0.message).await {
        Ok(()) => http::StatusCode::NO_CONTENT,
        Err(_error) => http::StatusCode::BAD_GATEWAY,
    }
}

pub fn notification_router<Sender>(
    state: NotificationServiceState<Sender>,
) -> AxumNotificationRouter
where
    Sender: NotificationSender,
{
    AxumNotificationRouter::from(
        axum::Router::new()
            .route(
                constants_str::NOTIFICATIONS_PATH,
                axum::routing::post(send_notification::<Sender>),
            )
            .with_state(state),
    )
}

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
