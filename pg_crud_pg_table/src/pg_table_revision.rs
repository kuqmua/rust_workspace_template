#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    sqlx::Type,
    newtype::Display,
)]
#[sqlx(transparent)]
pub struct PgTableRevision(pub(super) i64);

impl TryFrom<String> for PgTableRevision {
    type Error = crate::pg_table_revision_try_from_string_error::PgTableRevisionTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed = value.parse::<i64>().map_err(|error| {
            crate::pg_table_revision_try_from_string_error::PgTableRevisionTryFromStringError::Invalid(crate::pg_table_revision_parse_int_error::PgTableRevisionParseIntError::from(error))
        })?;
        if parsed < constants_i64::ZERO {
            Err(crate::pg_table_revision_try_from_string_error::PgTableRevisionTryFromStringError::Negative)
        } else {
            Ok(Self(parsed))
        }
    }
}
