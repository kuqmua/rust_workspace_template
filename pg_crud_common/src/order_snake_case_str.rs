#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::Display,
)]
pub struct OrderSnakeCaseStr(crate::order_text_string::OrderTextString);

impl
    From<crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError>
    for OrderSnakeCaseStr
{
    fn from(
        value: crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError,
    ) -> Self {
        Self(crate::order_text_string::OrderTextString::from(value))
    }
}

impl TryFrom<String> for OrderSnakeCaseStr {
    type Error =
        crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::order_text_string::OrderTextString::try_from(value).map(Self)
    }
}
