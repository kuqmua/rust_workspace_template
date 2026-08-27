#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
pub(crate) const NOT_EMPTY_UNIQUE_VEC_MAX_LEN: usize = 10_000usize;
