const TO_ERR_STRING_VALUE_MAX_LEN: usize = 1_048_576;
to_err_string_macros::impl_to_err_string_with!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, f32, f64, bool, char => |v| v.to_string());
to_err_string_macros::impl_to_err_string_with!(reqwest::header::HeaderMap, http_body::SizeHint => |v| format!("{v:#?}"));
to_err_string_macros::impl_to_err_string_with!(
    http::header::ToStrError,
    axum::Error,
    usize,
    time::error::ComponentRange,
    sqlx::types::uuid::Error,
    std::io::Error,
    sqlx::Error,
    serde_json::Error,
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
    fn to_err_string(&self) -> ToErrStringValue;
}
#[derive(Debug, Clone, PartialEq, Eq, newtype::BoundedString, newtype::Newtype)]
#[bounded_string(
    max = TO_ERR_STRING_VALUE_MAX_LEN,
    description = "to error string value"
)]
#[newtype(as_ref_str, deref_target, display)]
pub struct ToErrStringValue(String);
impl ToErrStringValue {
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl<T> ToErrString for &T
where
    T: ToErrString + ?Sized,
{
    fn to_err_string(&self) -> ToErrStringValue {
        (*self).to_err_string()
    }
}
impl<T> ToErrString for Option<T>
where
    T: std::fmt::Debug,
{
    fn to_err_string(&self) -> ToErrStringValue {
        debug_to_string(self)
    }
}
impl<T, E> ToErrString for Result<T, E>
where
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    fn to_err_string(&self) -> ToErrStringValue {
        debug_to_string(self)
    }
}
to_err_string_macros::impl_to_err_string_as_ref_str!(String, str, std::borrow::Cow<'_, str>);
to_err_string_macros::impl_to_err_string_const!(
    tracing::dispatcher::SetGlobalDefaultError => str_constants::TRACING_PATH_DISPATCHER_PATH_SETGLOBALDEFAULTERROR,
    tracing::log::SetLoggerError => str_constants::TRACING_PATH_LOG_PATH_TRACING_PATH_LOG_PATH_SETLOGGERERROR,
);
#[derive(Debug, Clone, Copy)]
struct StaticStrToOwnedInput(pub &'static str);
fn debug_to_string<T>(v: &T) -> ToErrStringValue
where
    T: std::fmt::Debug,
{
    ToErrStringValue::try_from(format!("{v:?}")).unwrap_or_else(ToErrStringValue::from)
}
fn as_ref_str_to_owned<T>(v: &T) -> ToErrStringValue
where
    T: ?Sized + AsRef<str>,
{
    ToErrStringValue::try_from(v.as_ref().to_owned()).unwrap_or_else(ToErrStringValue::from)
}
fn static_str_to_owned(v: StaticStrToOwnedInput) -> ToErrStringValue {
    ToErrStringValue::try_from(v.0.to_owned()).unwrap_or_else(ToErrStringValue::from)
}
#[cfg(test)]
mod tests {
    #[allow(clippy::single_call_fn)] // shared assertion keeps ToErrString behavior checks concise and consistent
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
}
