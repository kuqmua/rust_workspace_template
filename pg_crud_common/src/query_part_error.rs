#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    proc_macro_location_derive_location::Location,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[location_to_schema]
pub enum QueryPartError {
    CheckedAdd {
        location: location_lib::location::Location,
    },
    StringWrapperTryFromString {
        location: location_lib::location::Location,
        #[error_field_to_err_string_serde]
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
        value: crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError,
    ) -> Self {
        Self::StringWrapperTryFromString {
            location: proc_macro_location_bang::location!(),
            error: value,
        }
    }
}

impl to_err_string::to_err_string::ToErrString for QueryPartError {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(self.to_string())
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}
