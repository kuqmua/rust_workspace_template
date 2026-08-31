#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct PgTableIdempotencyKey(String);

impl TryFrom<String> for PgTableIdempotencyKey {
    type Error = crate::pg_table_idempotency_text_error::PgTableIdempotencyTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::validate_pg_table_idempotency_text::validate_pg_table_idempotency_text(value)
            .map(Self)
    }
}

impl From<uuid::Uuid> for PgTableIdempotencyKey {
    fn from(value: uuid::Uuid) -> Self {
        Self(value.to_string())
    }
}
