#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefTarget,
)]
pub struct PgTableIdempotencyBody(
    bounded_types::bounded_vec::BoundedVec<
        u8,
        { constants_usize::ZERO },
        { constants_usize::VALUE_1_048_576 },
    >,
);

impl TryFrom<Vec<u8>> for PgTableIdempotencyBody {
    type Error = crate::pg_table_idempotency_body_error::PgTableIdempotencyBodyError;

    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from(vec)
            .map(Self)
            .map_err(|_error| {
                crate::pg_table_idempotency_body_error::PgTableIdempotencyBodyError::TooLarge
            })
    }
}
