#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminIdentifierFilterError {
    #[error("administrator identifier filter is incomplete")]
    Incomplete,
    #[error("administrator identifier filter field is unsupported")]
    UnsupportedField,
    #[error("administrator identifier filter operation is unsupported")]
    UnsupportedOperation,
    #[error("administrator identifier filter value is invalid")]
    InvalidValue,
    #[error("administrator identifier filter end value is invalid")]
    InvalidEnd,
    #[error("administrator timestamp filter value is invalid: {0}")]
    InvalidTimestampValue(frontend_contract::form_value_error::FormValueError),
    #[error("administrator timestamp filter end value is invalid: {0}")]
    InvalidTimestampEnd(frontend_contract::form_value_error::FormValueError),
    #[error("administrator identifier filter has an unexpected end value")]
    UnexpectedEnd,
    #[error("administrator identifier filter could not be represented")]
    Representation(
        #[source] crate::admin_where_many_try_from_string_error::AdminWhereManyTryFromStringError,
    ),
}
