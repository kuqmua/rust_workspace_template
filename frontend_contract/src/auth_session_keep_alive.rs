#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthSessionKeepAlive {
    interval: AuthSessionRefreshIntervalDuration,
    next: Option<AuthSessionInstant>,
    state: AuthSessionRefreshState,
}

impl AuthSessionKeepAlive {
    pub fn begin(
        &mut self,
        now: AuthSessionInstant,
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
        now: AuthSessionInstant,
        outcome: AuthSessionRefreshOutcome,
    ) -> AuthSessionRefreshOutcome {
        self.state = AuthSessionRefreshState::Idle;
        self.next = match outcome {
            AuthSessionRefreshOutcome::Failed | AuthSessionRefreshOutcome::Refreshed => now
                .0
                .checked_add(self.interval.0)
                .map(AuthSessionInstant::from),
            AuthSessionRefreshOutcome::Rejected => None,
        };
        outcome
    }

    pub const fn mark_missing(&mut self) {
        self.next = None;
        self.state = AuthSessionRefreshState::Idle;
    }

    #[must_use]
    pub const fn new(interval: AuthSessionRefreshIntervalDuration) -> Self {
        Self {
            interval,
            next: None,
            state: AuthSessionRefreshState::Idle,
        }
    }
}

pub use crate::auth_session_instant::AuthSessionInstant;
pub use crate::auth_session_keep_alive_decision::AuthSessionKeepAliveDecision;
pub use crate::auth_session_keep_alive_error::AuthSessionKeepAliveError;
pub use crate::auth_session_presence::AuthSessionPresence;
pub use crate::auth_session_refresh_interval_duration::AuthSessionRefreshIntervalDuration;
pub use crate::auth_session_refresh_outcome::AuthSessionRefreshOutcome;
use crate::auth_session_refresh_state::AuthSessionRefreshState;

#[cfg(test)]
mod tests {
    #[test]
    fn refresh_is_single_flight_and_rejection_clears_schedule() {
        let interval = super::AuthSessionRefreshIntervalDuration::try_from(
            std::time::Duration::from_secs(60u64),
        )
        .expect(
            "99658ad5 refresh_is_single_flight_and_rejection_clears_schedule invariant must hold",
        );
        let now = super::AuthSessionInstant::from(std::time::Instant::now());
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
