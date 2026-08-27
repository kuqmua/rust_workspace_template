#[path = "auth_session_keep_alive/auth_session_instant.rs"]
mod auth_session_instant;
#[path = "auth_session_keep_alive/auth_session_keep_alive.rs"]
mod auth_session_keep_alive;
#[path = "auth_session_keep_alive/auth_session_keep_alive_decision.rs"]
mod auth_session_keep_alive_decision;
#[path = "auth_session_keep_alive/auth_session_keep_alive_error.rs"]
mod auth_session_keep_alive_error;
#[path = "auth_session_keep_alive/auth_session_presence.rs"]
mod auth_session_presence;
#[path = "auth_session_keep_alive/auth_session_refresh_interval_duration.rs"]
mod auth_session_refresh_interval_duration;
#[path = "auth_session_keep_alive/auth_session_refresh_outcome.rs"]
mod auth_session_refresh_outcome;
#[path = "auth_session_keep_alive/auth_session_refresh_state.rs"]
mod auth_session_refresh_state;

pub use auth_session_instant::AuthSessionInstant;
pub use auth_session_keep_alive::AuthSessionKeepAlive;
pub use auth_session_keep_alive_decision::AuthSessionKeepAliveDecision;
pub use auth_session_keep_alive_error::AuthSessionKeepAliveError;
pub use auth_session_presence::AuthSessionPresence;
pub use auth_session_refresh_interval_duration::AuthSessionRefreshIntervalDuration;
pub use auth_session_refresh_outcome::AuthSessionRefreshOutcome;
use auth_session_refresh_state::AuthSessionRefreshState;

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
