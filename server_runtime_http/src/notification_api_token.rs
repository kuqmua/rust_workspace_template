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

#[cfg(test)]
mod tests {
    #[test]
    fn test_notification_token_utf8_size_limits_and_debug_redaction() {
        assert!(
            [
                String::default(),
                constants_str::X.to_owned(),
                constants_str::X.repeat(4095usize),
                constants_str::X.repeat(4096usize),
                constants_str::X.repeat(4097usize),
                '\u{e9}'.to_string().repeat(2048usize),
                format!(
                    "{}{}",
                    '\u{e9}'.to_string().repeat(2048usize),
                    constants_str::X
                ),
            ]
            .into_iter()
            .all(|text| {
                let result =
                    crate::notification_api_token::NotificationApiToken::try_from(text.clone());
                if text.is_empty() {
                    result
                        == Err(
                            crate::notification_api_token_error::NotificationApiTokenError::Empty,
                        )
                } else if text.len() > 4096usize {
                    result
                        == Err(
                            crate::notification_api_token_error::NotificationApiTokenError::TooLong,
                        )
                } else {
                    result.is_ok_and(|token| {
                        bool::from(token.authorizes(
                            crate::notification_api_token_ref::NotificationApiTokenRef::from(
                                text.as_str(),
                            ),
                        )) && format!("{token:?}") == constants_str::NOTIFICATION_API_TOKEN_REDACTED
                    })
                }
            })
        );
    }

    #[test]
    fn test_notification_token_rejects_mismatch_at_every_position_and_length() {
        let text = constants_str::X.repeat(32usize);
        assert!(
            crate::notification_api_token::NotificationApiToken::try_from(text.clone()).is_ok_and(
                |token| {
                    let mismatches_rejected = (0usize..32usize).all(|position| {
                        let candidate = text
                            .chars()
                            .enumerate()
                            .map(
                                |(index, character)| {
                                    if index == position { 'y' } else { character }
                                },
                            )
                            .collect::<String>();
                        !bool::from(token.authorizes(
                            crate::notification_api_token_ref::NotificationApiTokenRef::from(
                                candidate.as_str(),
                            ),
                        ))
                    });
                    mismatches_rejected
                        && [
                            String::default(),
                            constants_str::X.repeat(31usize),
                            constants_str::X.repeat(33usize),
                            format!("{text}{}", '\0'),
                        ]
                        .into_iter()
                        .all(|candidate| {
                            !bool::from(token.authorizes(
                                crate::notification_api_token_ref::NotificationApiTokenRef::from(
                                    candidate.as_str(),
                                ),
                            ))
                        })
                }
            )
        );
    }
}
