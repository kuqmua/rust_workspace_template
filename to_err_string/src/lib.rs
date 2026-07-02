extern crate alloc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ErrorString(String);

pub trait ToErrString {
    #[must_use]
    fn to_err_string(&self) -> ErrorString;
}

impl AsRef<str> for ErrorString {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<ErrorString> for String {
    fn from(value: ErrorString) -> Self {
        value.0
    }
}

impl From<String> for ErrorString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

to_err_string_macros::impl_to_err_string_with_to_string!(
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    bool,
    char,
    std::io::Error,
);
to_err_string_macros::impl_to_err_string_with_as_ref_str!(String, str, alloc::borrow::Cow<'_, str>,);
to_err_string_macros::impl_to_err_string_with_static_message!(
    core::fmt::Error => "core::fmt::Error",
    core::convert::Infallible => "core::convert::Infallible",
);

impl<T> ToErrString for &T
where
    T: ToErrString + ?Sized,
{
    fn to_err_string(&self) -> ErrorString {
        (*self).to_err_string()
    }
}

impl<T> ToErrString for Option<T>
where
    T: core::fmt::Debug,
{
    fn to_err_string(&self) -> ErrorString {
        format!("{self:?}").into()
    }
}

impl<T, E> ToErrString for Result<T, E>
where
    T: core::fmt::Debug,
    E: core::fmt::Debug,
{
    fn to_err_string(&self) -> ErrorString {
        format!("{self:?}").into()
    }
}

#[cfg(test)]
mod tests {
    fn assert_to_err_string<ErrorValue, ExpectedText>(
        error_value: ErrorValue,
        expected_text: ExpectedText,
    ) -> Result<(), String>
    where
        ErrorValue: crate::ToErrString,
        ExpectedText: AsRef<str>,
    {
        let actual_text = error_value.to_err_string();
        if actual_text.as_ref() == expected_text.as_ref() {
            return Ok(());
        }
        Err(format!("{} != {}", actual_text.as_ref(), expected_text.as_ref()))
    }

    #[test]
    fn to_err_string_for_primitives_and_options() -> Result<(), String> {
        let signed_32_bit_integer: i32 = 42;
        let signed_128_bit_integer: i128 = 42;
        let signed_pointer_sized_integer: isize = 42;
        let unsigned_128_bit_integer: u128 = 42;
        assert_to_err_string(signed_32_bit_integer, "42")?;
        assert_to_err_string(signed_128_bit_integer, "42")?;
        assert_to_err_string(signed_pointer_sized_integer, "42")?;
        assert_to_err_string(unsigned_128_bit_integer, "42")?;
        assert_to_err_string(Some::<u8>(7), "Some(7)")?;
        assert_to_err_string(None::<u16>, "None")?;
        assert_to_err_string(true, "true")?;
        assert_to_err_string('x', "x")?;
        assert_to_err_string(Some(String::from("abc")), "Some(\"abc\")")
    }

    #[test]
    fn to_err_string_for_strings_and_str_refs() -> Result<(), String> {
        let owned = String::from("abc");
        let borrowed = "xyz";
        assert_to_err_string(owned, "abc")?;
        assert_to_err_string(borrowed, "xyz")?;
        assert_to_err_string(alloc::borrow::Cow::Borrowed("qwe"), "qwe")?;
        assert_to_err_string(alloc::borrow::Cow::<'_, str>::Owned(String::from("rty")), "rty")
    }

    #[test]
    fn to_err_string_for_result_values() -> Result<(), String> {
        assert_to_err_string(Result::<u8, u16>::Ok(5), "Ok(5)")?;
        assert_to_err_string(Result::<u8, &'static str>::Err("er"), "Err(\"er\")")
    }
}
