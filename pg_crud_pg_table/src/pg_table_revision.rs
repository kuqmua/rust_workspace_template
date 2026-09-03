#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    sqlx::Type,
    proc_macro_newtype_display::Display,
)]
#[sqlx(transparent)]
pub struct PgTableRevision(i64);

impl TryFrom<String> for PgTableRevision {
    type Error = crate::pg_table_revision_try_from_string_error::PgTableRevisionTryFromStringError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let parsed = string.parse::<i64>().map_err(|error| {
            crate::pg_table_revision_try_from_string_error::PgTableRevisionTryFromStringError::Invalid(crate::pg_table_revision_parse_int_error::PgTableRevisionParseIntError::from(error))
        })?;
        if parsed < constants_i64::ZERO {
            Err(crate::pg_table_revision_try_from_string_error::PgTableRevisionTryFromStringError::Negative)
        } else {
            Ok(Self(parsed))
        }
    }
}
