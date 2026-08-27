#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::*;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::DerefTarget, serde::Deserialize,
)]
#[serde(try_from = "Vec<PgType>")]
pub(super) struct GeneratePgTypes(pub(super) Vec<PgType>);

impl TryFrom<Vec<PgType>> for GeneratePgTypes {
    type Error = generate_pg_types_length_error::GeneratePgTypesLengthError;

    fn try_from(value: Vec<PgType>) -> Result<Self, Self::Error> {
        if value.len() > GENERATE_PG_TYPES_MAX_LEN {
            Err(generate_pg_types_length_error::GeneratePgTypesLengthError)
        } else {
            Ok(Self(value))
        }
    }
}
