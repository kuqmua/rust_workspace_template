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

#[cfg(test)]
mod tests {
    #[test]
    fn test_notification_message_boundaries_round_trip_and_transfer_request_ownership() {
        let unicode = '\u{e9}'.to_string().repeat(32_768usize);
        let mut oversized_unicode = unicode.clone();
        oversized_unicode.push('x');
        assert!(
            [
                (
                    String::new(),
                    Some(crate::notification_message_error::NotificationMessageError::Empty)
                ),
                ('\n'.to_string(), None),
                (constants_str::X.to_owned(), None),
                (constants_str::X.repeat(65_535usize), None),
                (constants_str::X.repeat(65_536usize), None),
                (
                    constants_str::X.repeat(65_537usize),
                    Some(crate::notification_message_error::NotificationMessageError::TooLong)
                ),
                (unicode, None),
                (
                    oversized_unicode,
                    Some(crate::notification_message_error::NotificationMessageError::TooLong)
                ),
            ]
            .into_iter()
            .all(|(text, expected_error)| {
                let converted =
                    crate::runtime_notification_message::RuntimeNotificationMessage::try_from(
                        text.clone(),
                    );
                let deserialized = serde_json::from_value::<
                    crate::runtime_notification_message::RuntimeNotificationMessage,
                >(serde_json::Value::String(text.clone()));
                if let Some(error) = expected_error {
                    converted == Err(error) && deserialized.is_err_and(|source| source.is_data())
                } else {
                    converted.is_ok_and(|message| {
                        message.as_ref() == text
                        && deserialized.is_ok_and(|decoded| decoded == message)
                        && serde_json::to_value(&message).is_ok_and(|json| json == text)
                        && crate::runtime_notification_message::RuntimeNotificationMessage::from(
                            crate::notification_request::NotificationRequest::new(message),
                        ).as_ref() == text
                    })
                }
            })
        );
    }
}
