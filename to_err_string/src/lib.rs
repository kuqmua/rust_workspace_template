to_err_string_macros::impl_to_err_string_with!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, f32, f64, bool, char => |v| v.to_string());
to_err_string_macros::impl_to_err_string_with!(reqwest::header::HeaderMap, http_body::SizeHint => |v| debug_alt_to_string(v));
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToErrStringValue(pub String);
impl From<String> for ToErrStringValue {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for ToErrStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for ToErrStringValue {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl std::ops::Deref for ToErrStringValue {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
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
    tracing::dispatcher::SetGlobalDefaultError => "tracing::dispatcher::SetGlobalDefaultEr",
    tracing::log::SetLoggerError => "tracing::log::tracing::log::SetLoggerError",
);
#[derive(Debug, Clone, Copy)]
struct StaticStrToOwnedInput(pub &'static str);
fn debug_alt_to_string<T>(v: &T) -> ToErrStringValue
where
    T: std::fmt::Debug,
{
    ToErrStringValue(format!("{v:#?}"))
}
fn debug_to_string<T>(v: &T) -> ToErrStringValue
where
    T: std::fmt::Debug,
{
    ToErrStringValue(format!("{v:?}"))
}
fn as_ref_str_to_owned<T>(v: &T) -> ToErrStringValue
where
    T: ?Sized + AsRef<str>,
{
    ToErrStringValue(v.as_ref().to_owned())
}
fn static_str_to_owned(v: StaticStrToOwnedInput) -> ToErrStringValue {
    ToErrStringValue(v.0.to_owned())
}
#[cfg(test)]
mod tests {
    #[allow(clippy::single_call_fn)] // shared assertion keeps ToErrString behavior checks concise and consistent
    fn assert_to_err_string(v: impl super::ToErrString, exp: &str) {
        assert_eq!(v.to_err_string().0, exp);
    }
    #[test]
    fn to_err_string_for_primitives_and_options() {
        assert_to_err_string(42i32, "42");
        assert_to_err_string(42i128, "42");
        assert_to_err_string(42isize, "42");
        assert_to_err_string(42u128, "42");
        assert_to_err_string(Some(7u8), "Some(7)");
        assert_to_err_string(None::<u16>, "None");
        assert_to_err_string(true, "true");
        assert_to_err_string('x', "x");
        assert_to_err_string(Some(String::from("abc")), "Some(\"abc\")");
    }
    #[test]
    fn to_err_string_for_strings_and_str_refs() {
        let owned = String::from("abc");
        let borrowed = "xyz";
        assert_to_err_string(owned, "abc");
        assert_to_err_string(borrowed, "xyz");
        assert_to_err_string(std::borrow::Cow::Borrowed("qwe"), "qwe");
        assert_to_err_string(
            std::borrow::Cow::<'_, str>::Owned(String::from("rty")),
            "rty",
        );
    }
    #[test]
    fn to_err_string_for_result_values() {
        assert_to_err_string(Result::<u8, u16>::Ok(5), "Ok(5)");
        assert_to_err_string(Result::<u8, &'static str>::Err("er"), "Err(\"er\")");
    }
}
