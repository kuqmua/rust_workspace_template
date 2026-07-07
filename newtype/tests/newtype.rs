#[cfg(test)]
mod tests {
    mod loc_lib {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub(crate) struct ToErrStringValue(pub String);
        pub(crate) trait ToErrString {
            fn to_err_string(&self) -> ToErrStringValue;
        }
    }
    #[derive(Debug, Clone, PartialEq, Eq, newtype::Newtype)]
    #[newtype(display, as_ref_str, deref, from, getter, to_err_string_as_ref_str)]
    struct StringValue(String);
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
        let v = StringValue::from(String::from("abc"));
        assert_eq!(v.to_string(), "abc");
        assert_eq!(v.as_ref(), "abc");
        assert_eq!(&*v, "abc");
        assert_eq!(GetStringValue::get_string_value(&v), "abc");
        assert_eq!(loc_lib::ToErrString::to_err_string(&v).0, "abc");
    }
    #[test]
    fn display_to_err_string_impl_is_generated() {
        let v = UsizeValue::from(42usize);
        assert_eq!(v.to_string(), "42");
        assert_eq!(loc_lib::ToErrString::to_err_string(&v).0, "42");
    }
    #[test]
    fn debug_to_err_string_impl_is_generated() {
        let v = DebugValue(vec![1, 2]);
        assert_eq!(loc_lib::ToErrString::to_err_string(&v).0, "[1, 2]");
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
