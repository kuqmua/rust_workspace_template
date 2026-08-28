#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Eq, PartialEq)]
pub struct NotificationApiToken(pub(super) String);

impl NotificationApiToken {
    #[must_use]
    pub fn authorizes(
        &self,
        candidate: super::NotificationApiTokenRef<'_>,
    ) -> super::NotificationApiTokenAuthorized {
        let maximum_len = self.0.len().max(candidate.0.len());
        let difference = (constants_usize::ZERO..maximum_len).fold(
            self.0.len() ^ candidate.0.len(),
            |acc, index| {
                acc | usize::from(
                    self.0
                        .as_bytes()
                        .get(index)
                        .copied()
                        .unwrap_or(constants_u8::ZERO)
                        ^ candidate
                            .0
                            .as_bytes()
                            .get(index)
                            .copied()
                            .unwrap_or(constants_u8::ZERO),
                )
            },
        );
        super::NotificationApiTokenAuthorized::from(difference == constants_usize::ZERO)
    }
}

impl std::fmt::Debug for NotificationApiToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::NOTIFICATION_API_TOKEN_REDACTED)
    }
}

impl TryFrom<String> for NotificationApiToken {
    type Error = super::NotificationApiTokenError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(Self::Error::Empty)
        } else if value.len() > constants_usize::VALUE_4_096 {
            Err(Self::Error::TooLong)
        } else {
            Ok(Self(value))
        }
    }
}
