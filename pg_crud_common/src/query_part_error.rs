#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    optimal_memory_layout::OptimalMemoryLayout,
)]
#[location_to_schema]
pub enum QueryPartError {
    CheckedAdd {
        location: location_lib::domain_types::Location,
    },
    StringWrapperTryFromString {
        location: location_lib::domain_types::Location,
        #[eo_to_err_string_serde]
        error: crate::domain_types::PgCrudStringWrapperTryFromStringError,
    },
    WriteIntoBuffer {
        location: location_lib::domain_types::Location,
    },
}

impl From<crate::domain_types::PgCrudStringWrapperTryFromStringError> for QueryPartError {
    fn from(error: crate::domain_types::PgCrudStringWrapperTryFromStringError) -> Self {
        Self::StringWrapperTryFromString {
            location: location_macros::location!(),
            error,
        }
    }
}
