#[cfg(test)]
mod tests {
    mod loc_lib {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub(crate) struct ToErrStringValue(String);
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub(crate) enum ToErrStringValueTryFromStringEr {
            TooLong,
        }
        impl std::fmt::Display for ToErrStringValueTryFromStringEr {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::TooLong => f.write_str("too long"),
                }
            }
        }
        impl From<ToErrStringValueTryFromStringEr> for ToErrStringValue {
            fn from(value: ToErrStringValueTryFromStringEr) -> Self {
                Self(value.to_string())
            }
        }
        impl TryFrom<String> for ToErrStringValue {
            type Error = ToErrStringValueTryFromStringEr;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                if value.len() > 1024 {
                    return Err(Self::Error::TooLong);
                }
                Ok(Self(value))
            }
        }
        impl AsRef<str> for ToErrStringValue {
            fn as_ref(&self) -> &str {
                self.0.as_str()
            }
        }
        pub(crate) trait ToErrString {
            fn to_err_string(&self) -> ToErrStringValue;
        }
    }
    const STRING_VALUE_MAX_LEN: usize = 1_048_576;
    #[derive(Debug, Clone, PartialEq, Eq, newtype::Newtype)]
    #[newtype(display, as_ref_str, deref, getter, to_err_string_as_ref_str)]
    struct StringValue(String);
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum StringValueTryFromStringEr {
        TooLong { len: usize, max: usize },
    }
    impl std::fmt::Display for StringValueTryFromStringEr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::TooLong { len, max } => {
                    write!(f, "string value length {len} exceeds maximum {max}")
                }
            }
        }
    }
    impl From<StringValueTryFromStringEr> for StringValue {
        fn from(value: StringValueTryFromStringEr) -> Self {
            Self(value.to_string())
        }
    }
    impl TryFrom<String> for StringValue {
        type Error = StringValueTryFromStringEr;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            if value.len() > STRING_VALUE_MAX_LEN {
                return Err(Self::Error::TooLong {
                    len: value.len(),
                    max: STRING_VALUE_MAX_LEN,
                });
            }
            Ok(Self(value))
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
    #[newtype(display, from, to_err_string)]
    struct UsizeValue(usize);
    #[derive(Debug, Clone, PartialEq, Eq, newtype::Newtype)]
    #[newtype(to_err_string_debug)]
    struct DebugValue(Vec<u8>);
    #[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::EnumFromStr)]
    enum SampleEnum {
        FirstValue,
        Second,
    }
    #[allow(dead_code)]
    fn dependency_markers(
        _: Option<proc_macro2::TokenStream>,
        _: Option<syn::Path>,
    ) -> proc_macro2::TokenStream {
        quote::quote! {}
    }
    #[test]
    fn string_newtype_impls_are_generated() {
        let v = StringValue(String::from("abc"));
        assert_eq!(v.to_string(), "abc");
        assert_eq!(v.as_ref(), "abc");
        assert_eq!(&*v, "abc");
        assert_eq!(GetStringValue::get_string_value(&v), "abc");
        assert_eq!(loc_lib::ToErrString::to_err_string(&v).as_ref(), "abc");
    }
    #[test]
    fn display_to_err_string_impl_is_generated() {
        let v = UsizeValue::from(42usize);
        assert_eq!(v.to_string(), "42");
        assert_eq!(loc_lib::ToErrString::to_err_string(&v).as_ref(), "42");
    }
    #[test]
    fn debug_to_err_string_impl_is_generated() {
        let v = DebugValue(vec![1, 2]);
        assert_eq!(loc_lib::ToErrString::to_err_string(&v).as_ref(), "[1, 2]");
    }
    #[test]
    fn enum_from_str_impl_is_generated() {
        assert_eq!(
            <SampleEnum as std::str::FromStr>::from_str("FIRST_VALUE"),
            Ok(SampleEnum::FirstValue)
        );
        assert_eq!(
            <SampleEnum as std::str::FromStr>::from_str("second"),
            Ok(SampleEnum::Second)
        );
    }
    #[test]
    fn enum_from_str_error_mentions_allowed_values() {
        let er = <SampleEnum as std::str::FromStr>::from_str("bad").expect_err("42d13f7a");
        assert_eq!(
            er,
            "Unknown value: bad. Allowed values: first_value, second"
        );
    }
}
