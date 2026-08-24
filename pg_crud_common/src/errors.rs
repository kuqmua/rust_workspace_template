#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
pub(crate) struct SqlxBoxDynError(#[source] sqlx::error::BoxDynError);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("failed to bind PostgreSQL query parameter")]
pub struct SqlxPostgresQueryBindError {
    #[source]
    source: SqlxBoxDynError,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
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
    fn to_err_string(&self) -> to_err_string::ErrorText {
        to_err_string::ErrorText::try_from(self.to_string())
            .unwrap_or_else(to_err_string::ErrorText::from)
    }
}
impl From<sqlx::error::BoxDynError> for SqlxPostgresQueryBindError {
    fn from(source: sqlx::error::BoxDynError) -> Self {
        Self {
            source: SqlxBoxDynError::from(source),
        }
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
    optimal_memory_layout::OptimalMemoryLayout,
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
pub fn mk_query_bind_err<Source>(source: Source) -> SqlxPostgresQueryBindError
where
    Source: std::error::Error + Send + Sync + 'static,
{
    let boxed: sqlx::error::BoxDynError = Box::new(source);
    SqlxPostgresQueryBindError::from(boxed)
}
#[cfg(test)]
mod tests {
    #[test]
    fn query_bind_error_preserves_its_source() {
        let error = super::mk_query_bind_err(std::io::Error::other(str_constants::ERROR));
        let source = std::error::Error::source(&error)
            .expect("c9d460e5 query_bind_error_preserves_its_source invariant must hold");

        assert_eq!(source.to_string(), str_constants::ERROR);
        assert_eq!(
            error.to_string(),
            "failed to bind PostgreSQL query parameter"
        );
        assert_eq!(
            source
                .source()
                .expect("4e5bcc6b query_bind_error_preserves_its_source invariant must hold")
                .to_string(),
            str_constants::ERROR
        );
    }
}
