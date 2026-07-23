pub const NOTIFICATION_API_BODY_MAX_BYTES: usize = 8_192;
const NOTIFICATION_MESSAGE_MAX_LEN: usize = 4_096;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
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
    Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, utoipa::ToSchema,
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

#[derive(Clone, Copy, Debug, frontend_contract::TypedRoute)]
#[typed_route(
    authentication = frontend_contract::AuthenticationRequirement::Public,
    error_policy = frontend_contract::RouteErrorPolicy::Default,
    method = frontend_contract::RouteMethod::Post,
    mutation = frontend_contract::RouteMutation::Mutating,
    obligations = frontend_contract::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "create_notification",
    path = "/notifications",
    request = CreateNotificationReq,
    request_body = frontend_contract::RouteRequestBody::Json,
    response = CreateNotificationRes,
    success_status = frontend_contract::SuccessStatus::Code201,
    transport = frontend_contract::PublicTransport
)]
pub struct CreateNotificationRoute;

#[derive(Clone, Copy, Debug, Eq, PartialEq, frontend_contract::RouteCatalog)]
#[route_catalog(
    family = NotificationRouteFamily,
    body_limit = NOTIFICATION_API_BODY_MAX_BYTES,
)]
pub enum NotificationRoute {
    #[route_catalog_route(CreateNotificationRoute)]
    Create,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
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
        .expect_err("6406611c");
        let _too_long_error = <super::NotificationMessage as serde::Deserialize>::deserialize(
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(
                "x".repeat(super::NOTIFICATION_MESSAGE_MAX_LEN + 1usize),
            ),
        )
        .expect_err("48d2019d");
    }
}
