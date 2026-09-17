#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct PgTableIdempotencyRoute(
    bounded_types::bounded_string::BoundedString<1usize, 1_024usize, false>,
);

impl TryFrom<String> for PgTableIdempotencyRoute {
    type Error = crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::Empty);
        }
        if value.len()
            > crate::pg_tbl_idempotency_route_max_bytes::PG_TBL_IDEMPOTENCY_ROUTE_MAX_BYTES
        {
            return Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::TooLong {
                actual_bytes: crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(value.len()),
                maximum_bytes: crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(
                    crate::pg_tbl_idempotency_route_max_bytes::PG_TBL_IDEMPOTENCY_ROUTE_MAX_BYTES,
                ),
            });
        }
        if value.starts_with('/') {
            bounded_types::bounded_string::BoundedString::try_from(value)
                .map(Self)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        actual_length,
                        maximum_length,
                    } => Self::Error::TooLong {
                        actual_bytes: crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(actual_length.get()),
                        maximum_bytes: crate::pg_table_idempotency_text_bytes::PgTableIdempotencyTextBytes::from(maximum_length.get()),
                    },
                    bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                        ..
                    } => Self::Error::Empty,
                })
        } else {
            Err(crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError::InvalidRoute)
        }
    }
}
