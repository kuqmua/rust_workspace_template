#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(try_from = "String")]
pub struct RuntimeNotificationMessage(String);

impl TryFrom<String> for RuntimeNotificationMessage {
    type Error = crate::notification_message_error::NotificationMessageError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.is_empty() {
            Err(Self::Error::Empty)
        } else if string.len() > 65_536usize {
            Err(Self::Error::TooLong)
        } else {
            Ok(Self(string))
        }
    }
}
