#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::*;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefTarget,
    newtype::IntoInnerFrom,
    serde::Deserialize,
)]
#[serde(try_from = "Vec<PgTypeRecord>")]
pub(super) struct GeneratePgTypeRecords(pub(super) Vec<PgTypeRecord>);

impl TryFrom<Vec<PgTypeRecord>> for GeneratePgTypeRecords {
    type Error = generate_pg_types_length_error::GeneratePgTypesLengthError;

    fn try_from(value: Vec<PgTypeRecord>) -> Result<Self, Self::Error> {
        if value.len() > GENERATE_PG_TYPES_MAX_LEN {
            Err(generate_pg_types_length_error::GeneratePgTypesLengthError)
        } else {
            Ok(Self(value))
        }
    }
}
