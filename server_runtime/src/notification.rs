#[derive(Clone, Eq, PartialEq)]
pub struct NotificationApiToken(String);

#[derive(Clone, Copy, Debug)]
pub struct NotificationApiTokenRef<'value_lt>(&'value_lt str);
impl<'value_lt> From<&'value_lt str> for NotificationApiTokenRef<'value_lt> {
    fn from(value: &'value_lt str) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NotificationApiTokenAuthorized(bool);
impl From<NotificationApiTokenAuthorized> for bool {
    fn from(value: NotificationApiTokenAuthorized) -> Self {
        value.0
    }
}

impl NotificationApiToken {
    #[must_use]
    pub fn authorizes(
        &self,
        candidate: NotificationApiTokenRef<'_>,
    ) -> NotificationApiTokenAuthorized {
        let maximum_len = self.0.len().max(candidate.0.len());
        let difference =
            (0usize..maximum_len).fold(self.0.len() ^ candidate.0.len(), |acc, index| {
                acc | usize::from(
                    self.0.as_bytes().get(index).copied().unwrap_or(0u8)
                        ^ candidate.0.as_bytes().get(index).copied().unwrap_or(0u8),
                )
            });
        NotificationApiTokenAuthorized(difference == 0usize)
    }
}

impl std::fmt::Debug for NotificationApiToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::NOTIFICATION_API_TOKEN_REDACTED)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NotificationApiTokenError {
    #[error("notification API token must not be empty")]
    Empty,
    #[error("notification API token exceeds maximum length")]
    TooLong,
}

impl TryFrom<String> for NotificationApiToken {
    type Error = NotificationApiTokenError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(Self::Error::Empty)
        } else if value.len() > 4096usize {
            Err(Self::Error::TooLong)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NotificationMessage(String);

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NotificationMessageError {
    #[error("notification message must not be empty")]
    Empty,
    #[error("notification message exceeds maximum length")]
    TooLong,
}

impl TryFrom<String> for NotificationMessage {
    type Error = NotificationMessageError;

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

impl AsRef<str> for NotificationMessage {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

pub trait NotificationSender: Clone + Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;

    fn send(
        &self,
        message: NotificationMessage,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

#[cfg(test)]
mod tests {
    #[test]
    fn api_token_debug_is_redacted() {
        let token = super::NotificationApiToken::try_from(String::from(
            str_constants::TEST_NOTIFICATION_API_TOKEN,
        ))
        .expect("9ac320d1");
        assert!(!format!("{token:?}").contains(str_constants::TEST_NOTIFICATION_API_TOKEN));
        assert!(bool::from(token.authorizes(
            super::NotificationApiTokenRef::from(str_constants::TEST_NOTIFICATION_API_TOKEN,)
        )));
    }
}
