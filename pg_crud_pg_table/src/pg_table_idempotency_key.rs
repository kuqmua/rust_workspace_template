#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct PgTableIdempotencyKey(pub(super) String);

impl TryFrom<String> for PgTableIdempotencyKey {
    type Error = super::PgTableIdempotencyTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(super::PgTableIdempotencyTextError::Empty)
        } else if value.len() > super::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES {
            Err(super::PgTableIdempotencyTextError::TooLong {
                actual_bytes: super::PgTableIdempotencyTextBytes::from(value.len()),
                maximum_bytes: super::PgTableIdempotencyTextBytes::from(
                    super::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES,
                ),
            })
        } else {
            Ok(Self(value))
        }
    }
}

impl From<uuid::Uuid> for PgTableIdempotencyKey {
    fn from(value: uuid::Uuid) -> Self {
        Self(value.to_string())
    }
}
