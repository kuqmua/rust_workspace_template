pub const NOTIFICATION_API_BODY_MAX_BYTES: usize = 8_192;
const NOTIFICATION_MESSAGE_MAX_LEN: usize = 4_096;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
#[serde(deny_unknown_fields)]
pub struct CreateNotificationReq {
    message: NotificationMessage,
}
impl CreateNotificationReq {
    #[must_use]
    pub fn into_message(self) -> NotificationMessage {
        self.message
    }
    #[must_use]
    pub const fn new(message: NotificationMessage) -> Self {
        Self { message }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct CreateNotificationRes {
    id: UuidNotificationId,
}
impl CreateNotificationRes {
    #[must_use]
    pub const fn id(&self) -> UuidNotificationId {
        self.id
    }
    #[must_use]
    pub const fn new(id: UuidNotificationId) -> Self {
        Self { id }
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
    utoipa::ToSchema,
)]
#[serde(try_from = "String")]
pub struct NotificationMessage(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    utoipa::ToSchema,
)]
#[serde(from = "uuid::Uuid")]
#[schema(value_type = String, format = "uuid")]
pub struct UuidNotificationId(uuid::Uuid);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::domain_types::AuthenticationRequirement::Public,
    error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default,
    method = frontend_contract::domain_types::RouteMethod::Post,
    mutation = frontend_contract::domain_types::RouteMutation::Mutating,
    obligations = frontend_contract::domain_types::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "create_notification",
    path = "/notifications",
    request = CreateNotificationReq,
    request_body = frontend_contract::domain_types::RouteRequestBody::Json,
    response = CreateNotificationRes,
    success_status = frontend_contract::domain_types::SuccessStatus::Code201,
    transport = frontend_contract::domain_types::PublicTransport
)]
pub struct CreateNotificationRoute;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    frontend_contract::domain_types::RouteCatalog,
)]
#[route_catalog(
    family = NotificationRouteFamily,
    body_limit = NOTIFICATION_API_BODY_MAX_BYTES,
)]
pub enum NotificationRoute {
    #[route_catalog_route(CreateNotificationRoute)]
    Create,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    frontend_contract::domain_types::RouteCatalog,
)]
#[route_catalog(
    family = NotificationOperationalRouteFamily,
    body_limit = NOTIFICATION_API_BODY_MAX_BYTES,
)]
pub enum NotificationOperationalRoute {
    #[route_catalog_route(
        contract = frontend_contract::domain_types::RouteContract::new(
            frontend_contract::domain_types::AuthenticationRequirement::Public,
            frontend_contract::domain_types::HttpMethod::Get,
            frontend_contract::domain_types::MutationKind::ReadOnly,
            frontend_contract::domain_types::ContractStr::from("/metrics"),
            frontend_contract::domain_types::SuccessStatus::Code200,
        ),
        path = "/metrics",
        exclude_from_family,
    )]
    Metrics,
    #[route_catalog_route(
        contract = frontend_contract::domain_types::RouteContract::new(
            frontend_contract::domain_types::AuthenticationRequirement::Public,
            frontend_contract::domain_types::HttpMethod::Get,
            frontend_contract::domain_types::MutationKind::ReadOnly,
            frontend_contract::domain_types::ContractStr::from("/openapi.json"),
            frontend_contract::domain_types::SuccessStatus::Code200,
        ),
        path = "/openapi.json",
        exclude_from_family,
    )]
    OpenApi,
}
impl frontend_contract::domain_types::RouteRegistrationContract for NotificationOperationalRoute {
    fn method(self) -> frontend_contract::domain_types::RouteMethod {
        frontend_contract::domain_types::RouteMethod::Get
    }
    fn path(self) -> frontend_contract::domain_types::RegisteredRoutePath {
        frontend_contract::domain_types::RegisteredRoutePath::from(match self {
            Self::Metrics => constants_str::METRICS,
            Self::OpenApi => constants_str::OPENAPI_JSON,
        })
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum NotificationMessageTryFromStringError {
    #[error("notification message must not be empty")]
    Empty,
    #[error("notification message exceeds its maximum length")]
    TooLong,
}
impl TryFrom<String> for NotificationMessage {
    type Error = NotificationMessageTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::Empty);
        }
        if value.len() > NOTIFICATION_MESSAGE_MAX_LEN {
            return Err(Self::Error::TooLong);
        }
        Ok(Self(value))
    }
}
#[cfg(test)]
mod tests {
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
    struct ClientTransport;
    impl frontend_contract::domain_types::Transport for ClientTransport {
        fn send(
            &self,
            _request: frontend_contract::domain_types::TransportRequest,
        ) -> impl Future<
            Output = Result<
                frontend_contract::domain_types::TransportResponse,
                frontend_contract::domain_types::TransportError,
            >,
        > + '_ {
            std::future::ready(Err(
                frontend_contract::domain_types::TransportError::default(),
            ))
        }
    }
    #[test]
    fn every_notification_route_has_named_route_and_client_functions() {
        assert_eq!(
            <super::NotificationRouteFamily as frontend_contract::domain_types::RouteFamily>::ROUTE_COUNT,
            constants_usize::ONE
        );
        assert_eq!(
            super::create_notification_route(),
            super::NotificationRoute::Create.contract().path()
        );
        assert_eq!(
            size_of_val(&super::create_notification_client::<ClientTransport>),
            constants_usize::ZERO
        );
        assert_eq!(
            <super::NotificationOperationalRouteFamily as frontend_contract::domain_types::RouteFamily>::ROUTE_COUNT,
            constants_usize::ZERO
        );
        assert_eq!(
            super::metrics_route(),
            super::NotificationOperationalRoute::Metrics
                .contract()
                .path()
        );
        assert_eq!(
            super::open_api_route(),
            super::NotificationOperationalRoute::OpenApi
                .contract()
                .path()
        );
        assert_eq!(
            size_of_val(&super::metrics_client::<ClientTransport>),
            constants_usize::ZERO
        );
        assert_eq!(
            size_of_val(&super::open_api_client::<ClientTransport>),
            constants_usize::ZERO
        );
    }
    #[test]
    fn notification_message_enforces_bounds() {
        assert!(matches!(
            super::NotificationMessage::try_from(String::new()),
            Err(super::NotificationMessageTryFromStringError::Empty)
        ));
        assert!(matches!(
            super::NotificationMessage::try_from("ready".to_owned()),
            Ok(_value)
        ));
        assert!(matches!(
            super::NotificationMessage::try_from("x".repeat(4_097usize)),
            Err(super::NotificationMessageTryFromStringError::TooLong)
        ));
    }

    #[test]
    fn notification_message_deserialization_enforces_bounds() {
        let _empty_error = <super::NotificationMessage as serde::Deserialize>::deserialize(
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(String::new()),
        )
        .expect_err(constants_str::VALUE_61A01611);
        let _too_long_error = <super::NotificationMessage as serde::Deserialize>::deserialize(
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(
                constants_str::X.repeat(super::NOTIFICATION_MESSAGE_MAX_LEN + constants_usize::ONE),
            ),
        )
        .expect_err(constants_str::VALUE_F2CF39E2);
    }
}
