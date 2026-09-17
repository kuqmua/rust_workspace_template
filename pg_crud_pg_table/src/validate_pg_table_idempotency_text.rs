pub(super) fn validate_pg_table_idempotency_text(
    string: String,
) -> Result<
    bounded_types::bounded_string::BoundedString<
        1usize,
        { crate::pg_tbl_idempotency_text_max_bytes::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES },
        false,
    >,
    crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError,
> {
    if string.is_empty() {
        Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::Empty)
    } else if string.len()
        > crate::pg_tbl_idempotency_text_max_bytes::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES
    {
        Err(
            crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::TooLong {
                actual_bytes:
                    crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                        string.len(),
                    ),
                maximum_bytes:
                    crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                        crate::pg_tbl_idempotency_text_max_bytes::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES,
                    ),
            },
        )
    } else {
        bounded_types::bounded_string::BoundedString::try_from(string).map_err(
            |source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    actual_length,
                    maximum_length,
                } => crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::TooLong {
                    actual_bytes:
                        crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                            actual_length.get(),
                        ),
                    maximum_bytes:
                        crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                            maximum_length.get(),
                        ),
                },
                bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    ..
                } => crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::Empty,
            },
        )
    }
}
