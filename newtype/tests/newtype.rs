const _: usize = str_constants::MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR.len();
#[cfg(test)]
mod tests {
    mod to_err_string {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub(crate) struct ToErrStringValue(String);
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub(crate) enum ToErrStringValueTryFromStringError {
            TooLong,
        }
        impl std::fmt::Display for ToErrStringValueTryFromStringError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::TooLong => f.write_str(str_constants::TOO_LONG),
                }
            }
        }
        impl From<ToErrStringValueTryFromStringError> for ToErrStringValue {
            fn from(value: ToErrStringValueTryFromStringError) -> Self {
                Self(value.to_string())
            }
        }
        impl TryFrom<String> for ToErrStringValue {
            type Error = ToErrStringValueTryFromStringError;
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
    const DESCRIBED_VALUE_MAX_LEN: usize = 2;
    const VALIDATE_LOWERCASE_ASCII: fn(&str) -> bool =
        |value| value.bytes().all(|byte| byte.is_ascii_lowercase());
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        newtype::AsRefStr,
        newtype::BoundedString,
        newtype::DerefTarget,
        newtype::Display,
        newtype::Getter,
        newtype::ToErrStringAsRefStr,
    )]
    #[bounded_string(max = STRING_VALUE_MAX_LEN)]
    struct StringValue(String);
    #[derive(
        Debug, Clone, PartialEq, Eq, newtype::Display, newtype::FromInner, newtype::ToErrString,
    )]
    struct UsizeValue(usize);
    #[derive(Debug, Clone, PartialEq, Eq, newtype::FromInner, newtype::ToErrStringDebug)]
    struct DebugValue(Vec<u8>);
    #[derive(
        Debug, Clone, Copy, PartialEq, Eq, newtype::AsRef, newtype::FromInner, newtype::IntoInner,
    )]
    struct InnerValue(u16);
    #[derive(
        Debug, Clone, PartialEq, Eq, newtype::AsSlice, newtype::FromInner, newtype::IntoVec,
    )]
    struct VecValue<T>(Vec<T>);
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        newtype::DerefInner,
        newtype::DerefMutInner,
        newtype::FromInner,
        newtype::IntoInnerFrom,
        newtype::IntoIterator,
    )]
    struct InnerVecValue<T>(Vec<T>);
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        newtype::DerefMutTarget,
        newtype::DerefTarget,
        newtype::FromInner,
    )]
    struct TargetVecValue(Vec<u8>);
    #[derive(Debug, Clone, PartialEq, Eq, newtype::BoundedString)]
    #[bounded_string(max = DESCRIBED_VALUE_MAX_LEN, description = "described value")]
    struct DescribedValue(String);
    #[derive(Debug, Clone, PartialEq, Eq, newtype::BoundedString)]
    #[bounded_string(max = 3usize, min = 1usize, chars, nul_free, serde, trim, utoipa)]
    struct RichValue(String);
    #[derive(Debug, Clone, PartialEq, Eq, newtype::BoundedString)]
    #[bounded_string(max = 3usize, validator = VALIDATE_LOWERCASE_ASCII)]
    struct ValidatedValue(String);
    #[derive(
        Debug, Clone, newtype::Display, newtype::FromInner, newtype::IntoInner, newtype::ToTokens,
    )]
    struct ProcMacro2TokenValue(proc_macro2::TokenStream);
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ReferentValue<'value_lt>(&'value_lt str, &'value_lt str);
    #[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
    struct ReferentValueRef<'value_lt>(&'value_lt ReferentValue<'value_lt>);
    #[derive(Debug, Clone, PartialEq, Eq, newtype::AsRefOwned, newtype::FromInner)]
    struct OwnedValue(Vec<u8>);
    #[derive(Debug, Clone, PartialEq, Eq, newtype::AsRefTarget, newtype::FromInner)]
    struct OwnedSliceValue(Vec<u8>);
    #[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
    struct SliceValueRef<'value_lt>(&'value_lt [u8]);
    #[derive(newtype::DebugTransparent, newtype::FromInner)]
    struct TransparentDebugValue(u16);
    #[derive(newtype::DebugRedacted, newtype::FromInner)]
    struct RedactedDebugValue(Vec<u8>);
    #[derive(Debug, newtype::Display, newtype::ErrorTransparent, newtype::FromInner)]
    struct StdTransparentErrorValue(std::io::Error);
    #[derive(newtype::AsMut, newtype::FromInner)]
    struct MutableValueRef<'value_lt>(&'value_lt mut u16);
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum CheckedTextError {
        TooLong,
    }
    #[derive(Debug, Clone, PartialEq, Eq, newtype::TryFrom)]
    #[try_from(validator = validate_checked_text)]
    struct CheckedText(String);
    #[derive(Debug, Clone, PartialEq, Eq, newtype::TryFrom)]
    #[try_from(error = CheckedTextError, validator = validate_checked_text)]
    struct ExplicitErrorCheckedText(String);
    #[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::EnumFromStr)]
    enum SampleEnum {
        FirstValue,
        Second,
    }
    #[allow(clippy::single_call_fn)] // validates named-function support in the Newtype derive
    fn validate_checked_text(value: &str) -> Result<(), CheckedTextError> {
        if value.len() > 2usize {
            Err(CheckedTextError::TooLong)
        } else {
            Ok(())
        }
    }
    #[allow(dead_code)]
    fn dependency_markers(
        _: Option<proc_macro2::TokenStream>,
        _: Option<syn::Path>,
        _: Option<workspace_macro_helpers::FirstIdentifier>,
    ) -> proc_macro2::TokenStream {
        quote::quote! {}
    }
    #[test]
    fn string_newtype_impls_are_generated() {
        let v = StringValue::try_from(String::from(str_constants::ABC_ALT_3)).expect("9d27b01c");
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
    fn try_from_validator_generates_checked_conversion() {
        assert_eq!(
            CheckedText::try_from(str_constants::AB.to_owned()),
            Ok(CheckedText(str_constants::AB.to_owned()))
        );
        assert_eq!(
            CheckedText::try_from(str_constants::ABC_ALT_3.to_owned()),
            Err(CheckedTextError::TooLong)
        );
    }
    #[test]
    fn try_from_validator_supports_explicit_error_type() {
        assert_eq!(
            ExplicitErrorCheckedText::try_from(str_constants::AB.to_owned()),
            Ok(ExplicitErrorCheckedText(str_constants::AB.to_owned()))
        );
        assert_eq!(
            ExplicitErrorCheckedText::try_from(str_constants::ABC_ALT_3.to_owned()),
            Err(CheckedTextError::TooLong)
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
        let v = DebugValue::from(vec![1, 2]);
        assert_eq!(
            to_err_string::ToErrString::to_err_string(&v).as_ref(),
            "[1, 2]"
        );
    }
    #[test]
    fn inner_accessors_are_generated() {
        let v = InnerValue::from(7);
        assert_eq!(*v.as_ref(), 7);
        assert_eq!(v.into_inner(), 7);
    }
    #[test]
    fn vec_accessors_are_generated() {
        let v = VecValue::from(vec![1i32, 2i32]);
        assert_eq!(v.as_slice(), [1i32, 2i32]);
        assert_eq!(v.into_vec(), vec![1i32, 2i32]);
    }
    #[test]
    fn inner_deref_and_from_are_generated() {
        let mut v = InnerVecValue::from(vec![1u8]);
        v.push(2);
        assert_eq!(&*v, &vec![1, 2]);
        assert_eq!(Vec::<u8>::from(v), vec![1, 2]);
    }
    #[test]
    fn consuming_into_iterator_is_generated() {
        let value = InnerVecValue::from(vec![1u8, 2u8]);
        assert_eq!(value.into_iter().collect::<Vec<u8>>(), vec![1u8, 2u8]);
    }
    #[test]
    fn target_deref_impls_are_generated() {
        let mut v = TargetVecValue::from(Vec::from(*b"lower"));
        v.make_ascii_uppercase();
        assert_eq!(&*v, b"LOWER");
    }
    #[test]
    fn redacted_debug_does_not_expose_inner_value() {
        let value = RedactedDebugValue::from(str_constants::SECRET.as_bytes().to_vec());
        let output = format!("{value:?}");
        assert!(output.contains(str_constants::REDACTED_ALT_3));
        assert!(!output.contains(str_constants::SECRET));
        assert_eq!(value.0, str_constants::SECRET.as_bytes());
    }
    #[test]
    fn transparent_error_exposes_inner_source() {
        let value = StdTransparentErrorValue::from(std::io::Error::other(str_constants::ERROR));
        assert_eq!(
            std::error::Error::source(&value).map(ToString::to_string),
            Some(String::from(str_constants::ERROR))
        );
    }
    #[test]
    fn mutable_reference_as_mut_is_generated() {
        let mut inner = 1u16;
        let mut value = MutableValueRef::from(&mut inner);
        *AsMut::<u16>::as_mut(&mut value) = 2u16;
        assert_eq!(inner, 2u16);
    }
    #[test]
    fn bounded_string_description_is_configurable() {
        let error = DescribedValue::try_from(String::from(str_constants::ABC_ALT_3))
            .expect_err(str_constants::VALUE_3DFCA278);
        assert_eq!(
            error.to_string(),
            "described value length 3 exceeds maximum 2"
        );
    }
    #[test]
    fn bounded_string_rich_policies_share_runtime_and_serde_validation() {
        assert_eq!(
            RichValue::try_from(String::from("  \u{430}\u{431}  ")),
            Ok(RichValue(String::from("\u{430}\u{431}")))
        );
        assert!(matches!(
            RichValue::try_from(String::from("   ")),
            Err(RichValueTryFromStringError::TooShort { len: 0, min: 1 })
        ));
        assert!(matches!(
            RichValue::try_from(String::from("abcd")),
            Err(RichValueTryFromStringError::TooLong { len: 4, max: 3 })
        ));
        assert!(matches!(
            RichValue::try_from(String::from("a\0b")),
            Err(RichValueTryFromStringError::ContainsNul)
        ));
        assert_eq!(
            serde_json::from_str::<RichValue>("\"  \\u0430\\u0431  \"").expect("1d3222b1"),
            RichValue(String::from("\u{430}\u{431}"))
        );
        let _error = serde_json::from_str::<RichValue>(str_constants::ABCD)
            .expect_err(str_constants::C0E03C6D);
    }
    #[test]
    fn bounded_string_openapi_limits_match_runtime_limits() {
        let schema = <RichValue as utoipa::ToSchema>::schema().1;
        let json = serde_json::to_value(schema).expect("756f3fe9");
        assert_eq!(json.get("minLength"), Some(&serde_json::json!(1usize)));
        assert_eq!(json.get("maxLength"), Some(&serde_json::json!(3usize)));
    }
    #[test]
    fn bounded_string_small_input_space_matches_reference_model() {
        let alphabet = ['a', ' ', '\0'];
        let all_match = alphabet
            .into_iter()
            .flat_map(|first| {
                alphabet.into_iter().flat_map(move |second| {
                    alphabet.into_iter().flat_map(move |third| {
                        alphabet
                            .into_iter()
                            .map(move |fourth| [first, second, third, fourth])
                    })
                })
            })
            .all(|chars| {
                let value = chars.into_iter().collect::<String>();
                let normalized = value.trim();
                let expected_ok = !normalized.contains('\0')
                    && (1usize..=3usize).contains(&normalized.chars().count());
                RichValue::try_from(value).is_ok() == expected_ok
            });
        assert!(all_match);
    }
    #[test]
    fn bounded_string_custom_validator_is_applied() {
        assert_eq!(
            ValidatedValue::try_from(String::from(str_constants::ABC_ALT_3)),
            Ok(ValidatedValue(String::from(str_constants::ABC_ALT_3)))
        );
        assert!(matches!(
            ValidatedValue::try_from(String::from(str_constants::GET)),
            Err(ValidatedValueTryFromStringError::InvalidValue)
        ));
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
        let inner = ReferentValue(str_constants::LEFT, str_constants::RIGHT);
        let v = ReferentValueRef::from(&inner);
        assert_eq!(AsRef::<ReferentValue<'_>>::as_ref(&v), &inner);
    }
    #[test]
    fn owned_inner_impls_are_generated() {
        let v = OwnedValue::from(vec![3, 5, 8]);
        assert_eq!(AsRef::<Vec<u8>>::as_ref(&v), &vec![3, 5, 8]);
    }
    #[test]
    fn owned_and_borrowed_slice_impls_are_generated() {
        let bytes = vec![3u8, 5u8, 8u8];
        let owned = OwnedSliceValue::from(bytes.clone());
        let borrowed = SliceValueRef::from(bytes.as_slice());
        assert_eq!(AsRef::<[u8]>::as_ref(&owned), bytes.as_slice());
        assert_eq!(AsRef::<[u8]>::as_ref(&borrowed), bytes.as_slice());
    }
    #[test]
    fn transparent_debug_forwards_inner_format() {
        assert_eq!(format!("{:?}", TransparentDebugValue(17)), "17");
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
        let error = <SampleEnum as std::str::FromStr>::from_str(str_constants::BAD)
            .expect_err(str_constants::VALUE_42D13F7A);
        assert_eq!(
            error,
            "Unknown value: bad. Allowed values: first_value, second"
        );
    }
}
