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
pub struct RuntimeNotificationMessage(String);

impl TryFrom<String> for RuntimeNotificationMessage {
    type Error = crate::notification_message_error::NotificationMessageError;

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
