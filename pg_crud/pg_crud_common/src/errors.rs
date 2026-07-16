#[derive(Debug, Clone, PartialEq, Eq, optml::Optml, newtype::Display)]
pub struct SqlxPostgresQueryBindError(String);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    utoipa::ToSchema,
)]
pub enum PgCrudStringWrapperTryFromStringError {
    #[error("string wrapper length {len} exceeds maximum {max}")]
    TooLong { len: usize, max: usize },
}
impl to_err_string::ToErrString for PgCrudStringWrapperTryFromStringError {
    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
        to_err_string::ToErrStringValue::try_from(self.to_string())
            .unwrap_or_else(to_err_string::ToErrStringValue::from)
    }
}
impl From<PgCrudStringWrapperTryFromStringError> for SqlxPostgresQueryBindError {
    fn from(value: PgCrudStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl SqlxPostgresQueryBindError {
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for SqlxPostgresQueryBindError {
    type Error = PgCrudStringWrapperTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: crate::PG_CRUD_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    optml::Optml,
)]
#[location_to_schema]
pub enum QueryPartError {
    CheckedAdd {
        location: location_lib::location::Location,
    },
    StringWrapperTryFromString {
        location: location_lib::location::Location,
        #[eo_to_err_string_serde]
        error: PgCrudStringWrapperTryFromStringError,
    },
    WriteIntoBuffer {
        location: location_lib::location::Location,
    },
}
impl From<PgCrudStringWrapperTryFromStringError> for QueryPartError {
    fn from(error: PgCrudStringWrapperTryFromStringError) -> Self {
        Self::StringWrapperTryFromString {
            location: location_macros::location!(),
            error,
        }
    }
}
