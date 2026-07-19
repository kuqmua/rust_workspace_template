const NOTIFICATION_MESSAGE_MAX_LEN: usize = 4_096;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct NotificationMessage(String);
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct UuidNotificationId(uuid::Uuid);

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
impl AsRef<str> for NotificationMessage {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<uuid::Uuid> for UuidNotificationId {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}
impl From<UuidNotificationId> for uuid::Uuid {
    fn from(value: UuidNotificationId) -> Self {
        value.0
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
}
