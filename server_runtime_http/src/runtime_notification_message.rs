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
pub struct RuntimeNotificationMessage(
    bounded_types::bounded_string::BoundedString<1usize, 65_536usize, false>,
);

impl TryFrom<String> for RuntimeNotificationMessage {
    type Error = crate::notification_message_error::NotificationMessageError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(Self::Error::Empty)
        } else if value.len() > 65_536usize {
            Err(Self::Error::TooLong)
        } else {
            bounded_types::bounded_string::BoundedString::try_from(value)
                .map(Self)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        ..
                    } => Self::Error::TooLong,
                    bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                        ..
                    } => Self::Error::Empty,
                })
        }
    }
}
impl From<crate::notification_request::NotificationRequest> for RuntimeNotificationMessage {
    fn from(value: crate::notification_request::NotificationRequest) -> Self {
        value.into_message()
    }
}
