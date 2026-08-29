#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct PgTableIdempotencyActor(pub(super) String);

impl TryFrom<String> for PgTableIdempotencyActor {
    type Error = crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::Empty)
        } else if value.len()
            > crate::pg_tbl_idempotency_text_max_bytes::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES
        {
            Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::TooLong {
                actual_bytes: crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(value.len()),
                maximum_bytes: crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                    crate::pg_tbl_idempotency_text_max_bytes::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES,
                ),
            })
        } else {
            Ok(Self(value))
        }
    }
}
