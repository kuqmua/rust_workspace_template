const ERROR_TEXT_MAX_LEN: usize = 1_048_576;
to_err_string_macros::impl_to_err_string_with!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, f32, f64, bool, char => |v| v.to_string());
#[cfg(not(target_arch = "wasm32"))]
to_err_string_macros::impl_to_err_string_with!(reqwest::header::HeaderMap, http_body::SizeHint => |v| format!("{v:#?}"));
to_err_string_macros::impl_to_err_string_with!(
    usize,
    std::io::Error,
    serde_json::Error
    => |v| v.to_string()
);
#[cfg(not(target_arch = "wasm32"))]
to_err_string_macros::impl_to_err_string_with!(
    http::header::ToStrError,
    axum::Error,
    time::error::ComponentRange,
    sqlx::types::uuid::Error,
    sqlx::Error,
    reqwest::Error,
    reqwest::StatusCode,
    axum::extract::rejection::JsonDataError,
    sqlx::migrate::MigrateError,
    axum::extract::rejection::JsonSyntaxError,
    axum::extract::rejection::JsonRejection,
    sqlx::types::chrono::NaiveTime,
    sqlx::types::chrono::NaiveDate,
    sqlx::types::chrono::NaiveDateTime,
    sqlx::types::time::Time,
    sqlx::types::time::PrimitiveDateTime,
    sqlx::types::Decimal,
    sqlx::types::BigDecimal
    => |v| v.to_string()
);
pub trait ToErrString {
    fn to_err_string(&self) -> ErrorText;
}
#[derive(
    optml::Optml,
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    newtype::BoundedString,
    newtype::AsRefStr,
    newtype::DerefTarget,
    newtype::Display,
    newtype::IntoInner,
)]
#[bounded_string(
    max = ERROR_TEXT_MAX_LEN,
    serde,
    description = "error text"
)]
pub struct ErrorText(String);
impl<T> ToErrString for &T
where
    T: ToErrString + ?Sized,
{
    fn to_err_string(&self) -> ErrorText {
        (*self).to_err_string()
    }
}
impl<T> ToErrString for Option<T>
where
    T: std::fmt::Debug,
{
    fn to_err_string(&self) -> ErrorText {
        debug_to_string(self)
    }
}
impl<T, E> ToErrString for Result<T, E>
where
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    fn to_err_string(&self) -> ErrorText {
        debug_to_string(self)
    }
}
to_err_string_macros::impl_to_err_string_as_ref_str!(String, str, std::borrow::Cow<'_, str>);
to_err_string_macros::impl_to_err_string_const!(
    tracing::dispatcher::SetGlobalDefaultError => str_constants::TRACING_PATH_DISPATCHER_PATH_SETGLOBALDEFAULTERROR,
    tracing::log::SetLoggerError => str_constants::TRACING_PATH_LOG_PATH_TRACING_PATH_LOG_PATH_SETLOGGERERROR,
);
#[derive(optml::Optml, Debug, Clone, Copy, newtype::FromInner)]
struct StaticStrToOwnedInput(&'static str);
fn debug_to_string<T>(v: &T) -> ErrorText
where
    T: std::fmt::Debug,
{
    ErrorText::try_from(format!("{v:?}")).unwrap_or_else(ErrorText::from)
}
fn as_ref_str_to_owned<T>(v: &T) -> ErrorText
where
    T: ?Sized + AsRef<str>,
{
    ErrorText::try_from(v.as_ref().to_owned()).unwrap_or_else(ErrorText::from)
}
fn static_str_to_owned(v: StaticStrToOwnedInput) -> ErrorText {
    ErrorText::try_from(v.0.to_owned()).unwrap_or_else(ErrorText::from)
}
#[cfg(test)]
mod tests {
    fn assert_to_err_string(v: impl super::ToErrString, exp: &str) {
        assert_eq!(v.to_err_string().as_ref(), exp);
    }
    #[test]
    fn to_err_string_for_primitives_and_options() {
        assert_to_err_string(42i32, str_constants::VALUE_42);
        assert_to_err_string(42i128, str_constants::VALUE_42);
        assert_to_err_string(42isize, str_constants::VALUE_42);
        assert_to_err_string(42u128, str_constants::VALUE_42);
        assert_to_err_string(Some(7u8), str_constants::SOME_7);
        assert_to_err_string(None::<u16>, str_constants::NONE);
        assert_to_err_string(true, str_constants::TRUE);
        assert_to_err_string('x', str_constants::X);
        assert_to_err_string(
            Some(String::from(str_constants::ABC_ALT_3)),
            str_constants::SOME_ABC,
        );
    }
    #[test]
    fn to_err_string_for_strings_and_str_refs() {
        let owned = String::from(str_constants::ABC_ALT_3);
        let borrowed = str_constants::XYZ;
        assert_to_err_string(owned, str_constants::ABC_ALT_3);
        assert_to_err_string(borrowed, str_constants::XYZ);
        assert_to_err_string(
            std::borrow::Cow::Borrowed(str_constants::QWE),
            str_constants::QWE,
        );
        assert_to_err_string(
            std::borrow::Cow::<'_, str>::Owned(String::from(str_constants::RTY)),
            str_constants::RTY,
        );
    }
    #[test]
    fn to_err_string_for_result_values() {
        assert_to_err_string(Result::<u8, u16>::Ok(5), str_constants::OK_5);
        assert_to_err_string(
            Result::<u8, &'static str>::Err(str_constants::CONFIG_TRACING_ERROR),
            str_constants::ERR_ERROR,
        );
    }
    #[test]
    #[cfg_attr(
        miri,
        ignore = "oversized JSON serialization is covered natively and is prohibitively slow under interpretation"
    )]
    fn error_text_owns_the_shared_length_invariant() {
        let valid =
            super::ErrorText::try_from(String::from(str_constants::ERROR)).expect("11a745a8");
        assert_eq!(valid.as_ref(), str_constants::ERROR);

        let oversized = "x".repeat(super::ERROR_TEXT_MAX_LEN.saturating_add(1usize));
        let _conversion_error =
            super::ErrorText::try_from(oversized.clone()).expect_err("06920f8a");
        let serialized = serde_json::to_string(&oversized).expect("fe92c1a6");
        let _deserialization_error =
            serde_json::from_str::<super::ErrorText>(serialized.as_str()).expect_err("a21a0577");
    }
}
