#[path = "admin_joined_text.rs"]
mod admin_joined_text;
#[path = "admin_joined_text_try_from_string_error.rs"]
mod admin_joined_text_try_from_string_error;
#[path = "join_text.rs"]
mod join_text;

pub(crate) use admin_joined_text::AdminJoinedText;
pub(crate) use admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError;
pub(crate) use join_text::join_text;
