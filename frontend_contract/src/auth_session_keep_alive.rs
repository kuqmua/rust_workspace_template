#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthSessionKeepAlive {
    interval: crate::auth_session_refresh_interval_duration::AuthSessionRefreshIntervalDuration,
    next: Option<crate::auth_session_instant::AuthSessionInstant>,
    state: crate::auth_session_refresh_state::AuthSessionRefreshState,
}

impl AuthSessionKeepAlive {
    pub fn begin(
        &mut self,
        now: crate::auth_session_instant::AuthSessionInstant,
        presence: crate::auth_session_presence::AuthSessionPresence,
    ) -> crate::auth_session_keep_alive_decision::AuthSessionKeepAliveDecision {
        if presence == crate::auth_session_presence::AuthSessionPresence::Missing {
            self.mark_missing();
            return crate::auth_session_keep_alive_decision::AuthSessionKeepAliveDecision::SkipMissing;
        }
        if self.state == crate::auth_session_refresh_state::AuthSessionRefreshState::Running {
            return crate::auth_session_keep_alive_decision::AuthSessionKeepAliveDecision::SkipAlreadyRunning;
        }
        if let Some(next) = self.next
            && now.0 < next.0
        {
            return crate::auth_session_keep_alive_decision::AuthSessionKeepAliveDecision::SkipNotDue { next };
        }
        self.state = crate::auth_session_refresh_state::AuthSessionRefreshState::Running;
        crate::auth_session_keep_alive_decision::AuthSessionKeepAliveDecision::RefreshNow
    }

    pub fn finish(
        &mut self,
        now: crate::auth_session_instant::AuthSessionInstant,
        outcome: crate::auth_session_refresh_outcome::AuthSessionRefreshOutcome,
    ) -> crate::auth_session_refresh_outcome::AuthSessionRefreshOutcome {
        self.state = crate::auth_session_refresh_state::AuthSessionRefreshState::Idle;
        self.next = match outcome {
            crate::auth_session_refresh_outcome::AuthSessionRefreshOutcome::Failed
            | crate::auth_session_refresh_outcome::AuthSessionRefreshOutcome::Refreshed => now
                .0
                .checked_add(self.interval.0)
                .map(crate::auth_session_instant::AuthSessionInstant::from),
            crate::auth_session_refresh_outcome::AuthSessionRefreshOutcome::Rejected => None,
        };
        outcome
    }

    pub const fn mark_missing(&mut self) {
        self.next = None;
        self.state = crate::auth_session_refresh_state::AuthSessionRefreshState::Idle;
    }

    #[must_use]
    pub const fn new(
        interval: crate::auth_session_refresh_interval_duration::AuthSessionRefreshIntervalDuration,
    ) -> Self {
        Self {
            interval,
            next: None,
            state: crate::auth_session_refresh_state::AuthSessionRefreshState::Idle,
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn refresh_is_single_flight_and_rejection_clears_schedule() {
        let interval = crate::auth_session_refresh_interval_duration::AuthSessionRefreshIntervalDuration::try_from(
            std::time::Duration::from_secs(60u64),
        )
        .expect(
            "99658ad5 refresh_is_single_flight_and_rejection_clears_schedule invariant must hold",
        );
        let now = crate::auth_session_instant::AuthSessionInstant::from(std::time::Instant::now());
        let mut keep_alive = super::AuthSessionKeepAlive::new(interval);
        assert_eq!(
            keep_alive.begin(
                now,
                crate::auth_session_presence::AuthSessionPresence::Present
            ),
            crate::auth_session_keep_alive_decision::AuthSessionKeepAliveDecision::RefreshNow
        );
        assert_eq!(
            keep_alive.begin(now, crate::auth_session_presence::AuthSessionPresence::Present),
            crate::auth_session_keep_alive_decision::AuthSessionKeepAliveDecision::SkipAlreadyRunning
        );
        assert_eq!(
            keep_alive.finish(
                now,
                crate::auth_session_refresh_outcome::AuthSessionRefreshOutcome::Rejected
            ),
            crate::auth_session_refresh_outcome::AuthSessionRefreshOutcome::Rejected
        );
        assert_eq!(
            keep_alive.begin(
                now,
                crate::auth_session_presence::AuthSessionPresence::Missing
            ),
            crate::auth_session_keep_alive_decision::AuthSessionKeepAliveDecision::SkipMissing
        );
    }
}
