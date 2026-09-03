#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
#[serde(try_from = "String")]
pub struct NotificationMessage(String);

impl TryFrom<String> for NotificationMessage {
    type Error =
        crate::notification_message_try_from_string_error::NotificationMessageTryFromStringError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.is_empty() {
            return Err(Self::Error::Empty);
        }
        if string.len() > crate::notification_message_max_len::NOTIFICATION_MESSAGE_MAX_LEN {
            return Err(Self::Error::TooLong);
        }
        Ok(Self(string))
    }
}
