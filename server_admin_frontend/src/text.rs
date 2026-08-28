pub(crate) use crate::admin_joined_text::AdminJoinedText;
pub(crate) use crate::admin_joined_text_try_from_string_error::AdminJoinedTextTryFromStringError;
pub(crate) use crate::join_text::join_text;

// Root-owned module compatibility wrappers.
pub(crate) mod admin_joined_text {
    pub use crate::admin_joined_text::*;
}
pub(crate) mod admin_joined_text_try_from_string_error {
    pub use crate::admin_joined_text_try_from_string_error::*;
}
pub(crate) mod join_text {
    pub use crate::join_text::*;
}
