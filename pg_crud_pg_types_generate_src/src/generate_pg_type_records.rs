#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefTarget,
    newtype::IntoInnerFrom,
    serde::Deserialize,
)]
#[serde(try_from = "Vec<crate::pg_type_record::PgTypeRecord>")]
pub(super) struct GeneratePgTypeRecords(pub(super) Vec<crate::pg_type_record::PgTypeRecord>);

impl TryFrom<Vec<crate::pg_type_record::PgTypeRecord>> for GeneratePgTypeRecords {
    type Error = crate::generate_pg_types_length_error::GeneratePgTypesLengthError;

    fn try_from(value: Vec<crate::pg_type_record::PgTypeRecord>) -> Result<Self, Self::Error> {
        if value.len() > crate::generate_pg_types_max_len::GENERATE_PG_TYPES_MAX_LEN {
            Err(crate::generate_pg_types_length_error::GeneratePgTypesLengthError::TooLarge)
        } else {
            Ok(Self(value))
        }
    }
}
