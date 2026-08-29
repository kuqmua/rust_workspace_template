pub(crate) use super::admin_joined_text::AdminJoinedText;
pub(crate) use super::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError;
pub(crate) use super::join_text::join_text;
// Root-owned module compatibility wrappers.
pub(crate) mod admin_joined_text {
    pub use super::super::admin_joined_text::*;
}
pub(crate) mod admin_joined_text_try_from_string_error {
    pub use super::super::admin_joined_text_try_from_string_error::*;
}
pub(crate) mod join_text {
    pub use super::super::join_text::*;
}
