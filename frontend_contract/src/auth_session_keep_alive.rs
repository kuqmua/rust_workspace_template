#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdAuthSessionInstant(std::time::Instant);
impl From<std::time::Instant> for StdAuthSessionInstant {
    fn from(value: std::time::Instant) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdAuthSessionRefreshInterval(std::time::Duration);
impl TryFrom<std::time::Duration> for StdAuthSessionRefreshInterval {
    type Error = AuthSessionKeepAliveError;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(AuthSessionKeepAliveError::ZeroInterval)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthSessionPresence {
    Missing,
    Present,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthSessionRefreshOutcome {
    Failed,
    Refreshed,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthSessionKeepAliveDecision {
    RefreshNow,
    SkipAlreadyRunning,
    SkipMissing,
    SkipNotDue { next: StdAuthSessionInstant },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum AuthSessionKeepAliveError {
    #[error("authentication session refresh interval must not be zero")]
    ZeroInterval,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AuthSessionRefreshState {
    Idle,
    Running,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthSessionKeepAlive {
    interval: StdAuthSessionRefreshInterval,
    next: Option<StdAuthSessionInstant>,
    state: AuthSessionRefreshState,
}
impl AuthSessionKeepAlive {
    pub fn begin(
        &mut self,
        now: StdAuthSessionInstant,
        presence: AuthSessionPresence,
    ) -> AuthSessionKeepAliveDecision {
        if presence == AuthSessionPresence::Missing {
            self.mark_missing();
            return AuthSessionKeepAliveDecision::SkipMissing;
        }
        if self.state == AuthSessionRefreshState::Running {
            return AuthSessionKeepAliveDecision::SkipAlreadyRunning;
        }
        if let Some(next) = self.next
            && now.0 < next.0
        {
            return AuthSessionKeepAliveDecision::SkipNotDue { next };
        }
        self.state = AuthSessionRefreshState::Running;
        AuthSessionKeepAliveDecision::RefreshNow
    }

    pub fn finish(
        &mut self,
        now: StdAuthSessionInstant,
        outcome: AuthSessionRefreshOutcome,
    ) -> AuthSessionRefreshOutcome {
        self.state = AuthSessionRefreshState::Idle;
        self.next = match outcome {
            AuthSessionRefreshOutcome::Failed | AuthSessionRefreshOutcome::Refreshed => now
                .0
                .checked_add(self.interval.0)
                .map(StdAuthSessionInstant),
            AuthSessionRefreshOutcome::Rejected => None,
        };
        outcome
    }

    pub const fn mark_missing(&mut self) {
        self.next = None;
        self.state = AuthSessionRefreshState::Idle;
    }

    #[must_use]
    pub const fn new(interval: StdAuthSessionRefreshInterval) -> Self {
        Self {
            interval,
            next: None,
            state: AuthSessionRefreshState::Idle,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn refresh_is_single_flight_and_rejection_clears_schedule() {
        let interval =
            super::StdAuthSessionRefreshInterval::try_from(std::time::Duration::from_secs(60u64))
                .expect("99658ad5");
        let now = super::StdAuthSessionInstant::from(std::time::Instant::now());
        let mut keep_alive = super::AuthSessionKeepAlive::new(interval);
        assert_eq!(
            keep_alive.begin(now, super::AuthSessionPresence::Present),
            super::AuthSessionKeepAliveDecision::RefreshNow
        );
        assert_eq!(
            keep_alive.begin(now, super::AuthSessionPresence::Present),
            super::AuthSessionKeepAliveDecision::SkipAlreadyRunning
        );
        assert_eq!(
            keep_alive.finish(now, super::AuthSessionRefreshOutcome::Rejected),
            super::AuthSessionRefreshOutcome::Rejected
        );
        assert_eq!(
            keep_alive.begin(now, super::AuthSessionPresence::Missing),
            super::AuthSessionKeepAliveDecision::SkipMissing
        );
    }
}
