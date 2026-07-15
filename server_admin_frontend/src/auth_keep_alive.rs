#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct StdAuthRefreshInstant(u64);
impl StdAuthRefreshInstant {
    #[cfg(test)]
    pub(crate) const fn after(self, delay: StdAuthRefreshRetryDelay) -> Self {
        Self(self.0.saturating_add(delay.0))
    }
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn now() -> Self {
        let milliseconds = web_sys::window()
            .and_then(|window| window.performance())
            .map_or(0f64, |performance| performance.now());
        let microseconds = std::time::Duration::try_from_secs_f64(milliseconds / 1_000f64)
            .ok()
            .and_then(|duration| u64::try_from(duration.as_micros()).ok())
            .unwrap_or(0u64);
        Self(microseconds)
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) const fn now() -> Self {
        Self(0u64)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct StdAuthRefreshRetryDelay(u64);
impl StdAuthRefreshRetryDelay {
    pub(crate) const DEFAULT: Self = Self(5_000_000u64);
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AuthRefreshBegin {
    Join,
    Rejected,
    Start,
    Wait,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AuthRefreshOutcome {
    Refreshed,
    Rejected,
    TemporaryFailure,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AuthRefreshPhase {
    Idle,
    Refreshing,
    Rejected,
    RetryAt(StdAuthRefreshInstant),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AuthRefreshState {
    phase: AuthRefreshPhase,
    retry_delay: StdAuthRefreshRetryDelay,
}
impl Default for AuthRefreshState {
    fn default() -> Self {
        Self {
            phase: AuthRefreshPhase::Idle,
            retry_delay: StdAuthRefreshRetryDelay::DEFAULT,
        }
    }
}
impl AuthRefreshState {
    pub(crate) const fn begin(&mut self, now: StdAuthRefreshInstant) -> AuthRefreshBegin {
        match self.phase {
            AuthRefreshPhase::Idle => {
                self.phase = AuthRefreshPhase::Refreshing;
                AuthRefreshBegin::Start
            }
            AuthRefreshPhase::Refreshing => AuthRefreshBegin::Join,
            AuthRefreshPhase::Rejected => AuthRefreshBegin::Rejected,
            AuthRefreshPhase::RetryAt(retry_at) if now.0 >= retry_at.0 => {
                self.phase = AuthRefreshPhase::Refreshing;
                AuthRefreshBegin::Start
            }
            AuthRefreshPhase::RetryAt(_) => AuthRefreshBegin::Wait,
        }
    }
    pub(crate) const fn finish(&mut self, outcome: AuthRefreshOutcome, now: StdAuthRefreshInstant) {
        self.phase = match outcome {
            AuthRefreshOutcome::Refreshed => AuthRefreshPhase::Idle,
            AuthRefreshOutcome::Rejected => AuthRefreshPhase::Rejected,
            AuthRefreshOutcome::TemporaryFailure => AuthRefreshPhase::RetryAt(
                StdAuthRefreshInstant(now.0.saturating_add(self.retry_delay.0)),
            ),
        };
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn rejection_is_terminal_without_retry_loop() {
        let now = super::StdAuthRefreshInstant::now();
        let mut state = super::AuthRefreshState::default();
        assert_eq!(state.begin(now), super::AuthRefreshBegin::Start);
        state.finish(super::AuthRefreshOutcome::Rejected, now);
        assert_eq!(state.begin(now), super::AuthRefreshBegin::Rejected);
    }
    #[test]
    fn simultaneous_callers_join_one_refresh() {
        let now = super::StdAuthRefreshInstant::now();
        let mut state = super::AuthRefreshState::default();
        assert_eq!(state.begin(now), super::AuthRefreshBegin::Start);
        assert_eq!(state.begin(now), super::AuthRefreshBegin::Join);
        state.finish(super::AuthRefreshOutcome::Refreshed, now);
        assert_eq!(state.begin(now), super::AuthRefreshBegin::Start);
    }
    #[test]
    fn temporary_failure_allows_one_attempt_after_injected_deadline() {
        let now = super::StdAuthRefreshInstant::now();
        let mut state = super::AuthRefreshState::default();
        assert_eq!(state.begin(now), super::AuthRefreshBegin::Start);
        state.finish(super::AuthRefreshOutcome::TemporaryFailure, now);
        assert_eq!(state.begin(now), super::AuthRefreshBegin::Wait);
        let retry_at = now.after(super::StdAuthRefreshRetryDelay::DEFAULT);
        assert_eq!(state.begin(retry_at), super::AuthRefreshBegin::Start);
        assert_eq!(state.begin(retry_at), super::AuthRefreshBegin::Join);
    }
}
