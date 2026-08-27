#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthSessionKeepAlive {
    interval: super::AuthSessionRefreshIntervalDuration,
    next: Option<super::AuthSessionInstant>,
    state: super::AuthSessionRefreshState,
}

impl AuthSessionKeepAlive {
    pub fn begin(
        &mut self,
        now: super::AuthSessionInstant,
        presence: super::AuthSessionPresence,
    ) -> super::AuthSessionKeepAliveDecision {
        if presence == super::AuthSessionPresence::Missing {
            self.mark_missing();
            return super::AuthSessionKeepAliveDecision::SkipMissing;
        }
        if self.state == super::AuthSessionRefreshState::Running {
            return super::AuthSessionKeepAliveDecision::SkipAlreadyRunning;
        }
        if let Some(next) = self.next
            && now.0 < next.0
        {
            return super::AuthSessionKeepAliveDecision::SkipNotDue { next };
        }
        self.state = super::AuthSessionRefreshState::Running;
        super::AuthSessionKeepAliveDecision::RefreshNow
    }

    pub fn finish(
        &mut self,
        now: super::AuthSessionInstant,
        outcome: super::AuthSessionRefreshOutcome,
    ) -> super::AuthSessionRefreshOutcome {
        self.state = super::AuthSessionRefreshState::Idle;
        self.next = match outcome {
            super::AuthSessionRefreshOutcome::Failed
            | super::AuthSessionRefreshOutcome::Refreshed => now
                .0
                .checked_add(self.interval.0)
                .map(super::AuthSessionInstant::from),
            super::AuthSessionRefreshOutcome::Rejected => None,
        };
        outcome
    }

    pub const fn mark_missing(&mut self) {
        self.next = None;
        self.state = super::AuthSessionRefreshState::Idle;
    }

    #[must_use]
    pub const fn new(interval: super::AuthSessionRefreshIntervalDuration) -> Self {
        Self {
            interval,
            next: None,
            state: super::AuthSessionRefreshState::Idle,
        }
    }
}
