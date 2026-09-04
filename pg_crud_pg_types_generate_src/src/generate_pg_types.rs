#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_deref_target::DerefTarget,
    serde::Deserialize,
)]
#[serde(try_from = "Vec<crate::pg_type_catalog_kind::PgTypeCatalogKind>")]
pub(super) struct GeneratePgTypes(Vec<crate::pg_type_catalog_kind::PgTypeCatalogKind>);

impl TryFrom<Vec<crate::pg_type_catalog_kind::PgTypeCatalogKind>> for GeneratePgTypes {
    type Error = crate::generate_pg_types_length_error::GeneratePgTypesLengthError;

    fn try_from(
        value: Vec<crate::pg_type_catalog_kind::PgTypeCatalogKind>,
    ) -> Result<Self, Self::Error> {
        if value.len() > crate::generate_pg_types_max_len::GENERATE_PG_TYPES_MAX_LEN {
            Err(crate::generate_pg_types_length_error::GeneratePgTypesLengthError::TooLarge)
        } else {
            Ok(Self(value))
        }
    }
}
