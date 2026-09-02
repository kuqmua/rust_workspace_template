#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    proc_macro_location::Location,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[location_to_schema]
pub enum QueryPartError {
    CheckedAdd {
        location: location_lib::location::Location,
    },
    StringWrapperTryFromString {
        location: location_lib::location::Location,
        #[eo_to_err_string_serde]
        error: crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError,
    },
    WriteIntoBuffer {
        location: location_lib::location::Location,
    },
}

impl
    From<crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError>
    for QueryPartError
{
    fn from(
        pg_crud_string_wrapper_try_from_string_error: crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError,
    ) -> Self {
        Self::StringWrapperTryFromString {
            location: proc_macro_location_bang::location!(),
            error: pg_crud_string_wrapper_try_from_string_error,
        }
    }
}
