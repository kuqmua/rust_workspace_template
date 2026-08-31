#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct PgTableIdempotencyMethod(String);

impl TryFrom<String> for PgTableIdempotencyMethod {
    type Error = crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::Empty);
        }
        if value.len() > crate::pg_tbl_idempotency_text_max_bytes::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES
        {
            return Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::TooLong {
                actual_bytes: crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(value.len()),
                maximum_bytes: crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                    crate::pg_tbl_idempotency_text_max_bytes::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES,
                ),
            });
        }
        if matches!(
            value.as_str(),
            constants_str::POST | constants_str::PATCH | constants_str::DELETE
        ) {
            Ok(Self(value))
        } else {
            Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::InvalidMethod)
        }
    }
}
