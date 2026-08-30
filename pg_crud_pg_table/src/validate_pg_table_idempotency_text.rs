pub(super) fn validate_pg_table_idempotency_text(
    value: String,
) -> Result<String, crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError> {
    if value.is_empty() {
        Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::Empty)
    } else if value.len()
        > crate::pg_tbl_idempotency_text_max_bytes::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES
    {
        Err(
            crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::TooLong {
                actual_bytes:
                    crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                        value.len(),
                    ),
                maximum_bytes:
                    crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                        crate::pg_tbl_idempotency_text_max_bytes::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES,
                    ),
            },
        )
    } else {
        Ok(value)
    }
}
