#[path = "admin_no_body.rs"]
mod admin_no_body;
#[path = "admin_session_identifier.rs"]
mod admin_session_identifier;
#[path = "admin_session_timestamp.rs"]
mod admin_session_timestamp;
#[path = "admin_session_view.rs"]
mod admin_session_view;
#[path = "admin_sessions_page.rs"]
mod admin_sessions_page;

pub use admin_no_body::AdminNoBody;
pub use admin_session_identifier::AdminSessionIdentifier;
pub use admin_session_timestamp::AdminSessionTimestamp;
pub use admin_session_view::AdminSessionView;
pub use admin_sessions_page::AdminSessionsPage;

#[cfg(test)]
#[path = "domain_types_sessions_tests.rs"]
mod tests;
