#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
pub(crate) const PG_CRUD_STRING_WRAPPER_MAX_LEN: usize = 1_048_576;
