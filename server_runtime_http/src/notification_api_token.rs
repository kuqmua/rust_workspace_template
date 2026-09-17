#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Eq, PartialEq)]
pub struct NotificationApiToken(
    bounded_types::bounded_string::BoundedString<1usize, 4_096usize, false>,
);

impl NotificationApiToken {
    #[must_use]
    pub fn authorizes(
        &self,
        notification_api_token_ref: crate::notification_api_token_ref::NotificationApiTokenRef<'_>,
    ) -> crate::notification_api_token_authorized::NotificationApiTokenAuthorized {
        let candidate_text = notification_api_token_ref.get();
        let maximum_len = self.0.as_str().len().max(candidate_text.len());
        let difference = (constants_usize::ZERO..maximum_len).fold(
            self.0.as_str().len() ^ candidate_text.len(),
            |acc, index| {
                acc | usize::from(
                    self.0
                        .as_bytes()
                        .get(index)
                        .copied()
                        .unwrap_or(constants_u8::ZERO)
                        ^ candidate_text
                            .as_bytes()
                            .get(index)
                            .copied()
                            .unwrap_or(constants_u8::ZERO),
                )
            },
        );
        crate::notification_api_token_authorized::NotificationApiTokenAuthorized::from(
            difference == constants_usize::ZERO,
        )
    }
}

impl std::fmt::Debug for NotificationApiToken {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(constants_str::NOTIFICATION_API_TOKEN_REDACTED)
    }
}

impl TryFrom<String> for NotificationApiToken {
    type Error = crate::notification_api_token_error::NotificationApiTokenError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(Self::Error::Empty)
        } else if value.len() > constants_usize::VALUE_4_096 {
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
