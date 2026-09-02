#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::DerefTarget,
    proc_macro_newtype::IntoInnerFrom,
    serde::Deserialize,
)]
#[serde(try_from = "Vec<crate::pg_type_record::PgTypeRecord>")]
pub(super) struct GeneratePgTypeRecords(Vec<crate::pg_type_record::PgTypeRecord>);

impl TryFrom<Vec<crate::pg_type_record::PgTypeRecord>> for GeneratePgTypeRecords {
    type Error = crate::generate_pg_types_length_error::GeneratePgTypesLengthError;

    fn try_from(vec: Vec<crate::pg_type_record::PgTypeRecord>) -> Result<Self, Self::Error> {
        if vec.len() > crate::generate_pg_types_max_len::GENERATE_PG_TYPES_MAX_LEN {
            Err(crate::generate_pg_types_length_error::GeneratePgTypesLengthError::TooLarge)
        } else {
            Ok(Self(vec))
        }
    }
}
