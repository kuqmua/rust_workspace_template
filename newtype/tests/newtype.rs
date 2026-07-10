#[cfg(test)]
mod tests {
    mod to_err_string {
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
    #[derive(Debug, Clone, PartialEq, Eq, newtype::BoundedString, newtype::Newtype)]
    #[bounded_string(max = STRING_VALUE_MAX_LEN)]
    #[newtype(deref, getter, to_err_string_as_ref_str)]
    struct StringValue(String);
    #[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
    #[newtype(display, from_inner, to_err_string)]
    struct UsizeValue(usize);
    #[derive(Debug, Clone, PartialEq, Eq, newtype::Newtype)]
    #[newtype(to_err_string_debug)]
    struct DebugValue(Vec<u8>);
    #[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
    #[newtype(as_ref, into_inner)]
    struct InnerValue(u16);
    #[derive(Debug, Clone, PartialEq, Eq, newtype::Newtype)]
    #[newtype(as_slice, into_vec)]
    struct VecValue<T>(Vec<T>);
    #[derive(Debug, Clone, newtype::Newtype)]
    #[newtype(display, from_inner, into_inner, to_tokens)]
    struct ProcMacro2TokenValue(proc_macro2::TokenStream);
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ReferentValue<'value_lt>(&'value_lt str, &'value_lt str);
    #[derive(Debug, Clone, Copy, newtype::Newtype)]
    #[newtype(as_ref_inner, from_inner)]
    struct ReferentValueRef<'value_lt>(&'value_lt ReferentValue<'value_lt>);
    #[derive(Debug, Clone, PartialEq, Eq, newtype::Newtype)]
    #[newtype(as_ref_owned, from_inner)]
    struct OwnedValue(Vec<u8>);
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
        assert_eq!(
            to_err_string::ToErrString::to_err_string(&v).as_ref(),
            "abc"
        );
        assert_eq!(
            StringValue::try_from("abc".to_owned()).map(|value| value.to_string()),
            Ok(String::from("abc"))
        );
    }
    #[test]
    fn display_to_err_string_impl_is_generated() {
        let v = UsizeValue::from(42usize);
        assert_eq!(v.to_string(), "42");
        assert_eq!(to_err_string::ToErrString::to_err_string(&v).as_ref(), "42");
    }
    #[test]
    fn debug_to_err_string_impl_is_generated() {
        let v = DebugValue(vec![1, 2]);
        assert_eq!(
            to_err_string::ToErrString::to_err_string(&v).as_ref(),
            "[1, 2]"
        );
    }
    #[test]
    fn inner_accessors_are_generated() {
        let v = InnerValue(7);
        assert_eq!(*v.as_ref(), 7);
        assert_eq!(v.into_inner(), 7);
    }
    #[test]
    fn vec_accessors_are_generated() {
        let v = VecValue(vec![1i32, 2i32]);
        assert_eq!(v.as_slice(), [1i32, 2i32]);
        assert_eq!(v.into_vec(), vec![1i32, 2i32]);
    }
    #[test]
    fn token_impls_are_generated() {
        let inner = quote::quote! { sample::path };
        let v = ProcMacro2TokenValue::from(inner.clone());
        assert_eq!(v.to_string(), "sample :: path");
        assert_eq!(quote::quote! { #v }.to_string(), inner.to_string());
        assert_eq!(v.into_inner().to_string(), inner.to_string());
    }
    #[test]
    fn reference_inner_impls_are_generated() {
        let inner = ReferentValue("left", "right");
        let v = ReferentValueRef::from(&inner);
        assert_eq!(AsRef::<ReferentValue<'_>>::as_ref(&v), &inner);
    }
    #[test]
    fn owned_inner_impls_are_generated() {
        let v = OwnedValue::from(vec![3, 5, 8]);
        assert_eq!(AsRef::<Vec<u8>>::as_ref(&v), &vec![3, 5, 8]);
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
