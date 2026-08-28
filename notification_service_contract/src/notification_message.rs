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

impl TryFrom<String> for NotificationMessage {
    type Error = crate::domain_types::NotificationMessageTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::Empty);
        }
        if value.len() > crate::domain_types::NOTIFICATION_MESSAGE_MAX_LEN {
            return Err(Self::Error::TooLong);
        }
        Ok(Self(value))
    }
}
