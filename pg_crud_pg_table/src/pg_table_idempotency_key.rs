#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct PgTableIdempotencyKey(
    bounded_types::bounded_string::BoundedString<
        1usize,
        { crate::pg_tbl_idempotency_text_max_bytes::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES },
        false,
    >,
);

impl TryFrom<String> for PgTableIdempotencyKey {
    type Error = crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::validate_pg_table_idempotency_text::validate_pg_table_idempotency_text(value)
            .map(Self)
    }
}
