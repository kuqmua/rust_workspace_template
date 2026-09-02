const _: usize = constants_str::MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR.len();
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_foundation::FromInner,
    proc_macro_newtype_foundation::GetInner,
)]
struct FoundationValue(bool);
const _: fn(bool) -> FoundationValue = FoundationValue::from;
const _: fn(FoundationValue) -> bool = FoundationValue::get;
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
struct GeneratedGettersFixture {
    value: u8,
}
const _: fn(&GeneratedGettersFixture) -> &u8 = GeneratedGettersFixture::get_value;
#[cfg(test)]
mod tests {
    #[allow(
        clippy::module_inception,
        reason = "the derive fixture must reproduce the external crate's `to_err_string::to_err_string` path contract"
    )]
    mod to_err_string {
        pub(crate) mod error_text {
            #[derive(
                proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, PartialEq, Eq,
            )]
            pub(crate) struct ErrorText(String);
            #[derive(
                proc_macro_optimal_memory_layout::OptimalMemoryLayout,
                Debug,
                Clone,
                Copy,
                PartialEq,
                Eq,
            )]
            pub(crate) enum ErrorTextTryFromStringError {
                TooLong,
            }
            impl std::fmt::Display for ErrorTextTryFromStringError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        Self::TooLong => f.write_str(constants_str::TOO_LONG),
                    }
                }
            }
            impl From<ErrorTextTryFromStringError> for ErrorText {
                fn from(value: ErrorTextTryFromStringError) -> Self {
                    Self(value.to_string())
                }
            }
            impl TryFrom<String> for ErrorText {
                type Error = ErrorTextTryFromStringError;
                fn try_from(value: String) -> Result<Self, Self::Error> {
                    if value.len() > 1024 {
                        return Err(Self::Error::TooLong);
                    }
                    Ok(Self(value))
                }
            }
            impl AsRef<str> for ErrorText {
                fn as_ref(&self) -> &str {
                    self.0.as_str()
                }
            }
        }
        pub(crate) mod to_err_string {
            pub(crate) trait ToErrString {
                fn to_err_string(&self) -> super::error_text::ErrorText;
            }
        }
    }
    const STRING_VALUE_MAX_LEN: usize = 1_048_576;
    const DESCRIBED_VALUE_MAX_LEN: usize = 2;
    const VALIDATE_LOWERCASE_ASCII: fn(&str) -> bool =
        |value| value.bytes().all(|byte| byte.is_ascii_lowercase());
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype::FromInner,
        proc_macro_newtype::UtoipaSchema,
    )]
    struct UtoipaSchemaDelegatedValue(
        bounded_types::bounded_vec::BoundedVec<u8, 0, { constants_usize::TWO }>,
    );
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype::FromInner,
        proc_macro_newtype::UtoipaSchema,
    )]
    #[utoipa_schema(bounded_types::bounded_vec::BoundedVec<u8, 0, { constants_usize::TWO }>)]
    struct UtoipaSchemaOverrideValue(Vec<u8>);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        PartialEq,
        Eq,
        proc_macro_newtype::AsRefStr,
        proc_macro_newtype::BoundedStringWrapper,
        proc_macro_newtype::BorrowStr,
        proc_macro_newtype::DerefTarget,
        proc_macro_newtype::Display,
        proc_macro_newtype::Accessor,
        proc_macro_newtype::ToErrStringAsRefStr,
    )]
    #[bounded_string(max = STRING_VALUE_MAX_LEN)]
    struct StringValue(
        bounded_types::bounded_string::BoundedString<0usize, { STRING_VALUE_MAX_LEN }, false>,
    );
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        PartialEq,
        Eq,
        proc_macro_newtype::Display,
        proc_macro_newtype::FromInner,
        proc_macro_newtype::ToErrString,
    )]
    struct UsizeValue(usize);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        PartialEq,
        Eq,
        proc_macro_newtype::FromInner,
        proc_macro_newtype::ToErrStringDebug,
    )]
    struct DebugValue(Vec<u8>);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        proc_macro_newtype::AsRef,
        proc_macro_newtype::BorrowOwned,
        proc_macro_newtype::FromInner,
        proc_macro_newtype::IntoInner,
    )]
    struct InnerValue(u16);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype::FromGetter,
        PartialEq,
        Debug,
    )]
    #[from_getter(source = GetterSource, getter = get)]
    struct FromGetterValue(u16);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::FromInner,
    )]
    struct GetterSource(u16);
    impl GetterSource {
        fn get(self) -> u16 {
            self.0
        }
    }
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Copy,
        proc_macro_newtype::FromInner,
        proc_macro_newtype::GetInner,
    )]
    struct GetInnerValueRef<'value_lt>(&'value_lt str);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Copy,
        proc_macro_newtype::FromInner,
        proc_macro_newtype::GetInner,
    )]
    struct GetInnerBool(bool);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        PartialEq,
        Eq,
        proc_macro_newtype::AsSlice,
        proc_macro_newtype::FromInner,
        proc_macro_newtype::IntoVec,
    )]
    struct VecValue<T>(Vec<T>);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        PartialEq,
        Eq,
        proc_macro_newtype::DerefInner,
        proc_macro_newtype::DerefMutInner,
        proc_macro_newtype::FromInner,
        proc_macro_newtype::IntoInnerFrom,
        proc_macro_newtype::IntoIterator,
    )]
    struct InnerVecValue<T>(Vec<T>);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        PartialEq,
        Eq,
        proc_macro_newtype::DerefMutTarget,
        proc_macro_newtype::DerefTarget,
        proc_macro_newtype::FromInner,
    )]
    struct TargetVecValue(Vec<u8>);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        PartialEq,
        Eq,
        proc_macro_newtype::BoundedStringWrapper,
    )]
    #[bounded_string(max = DESCRIBED_VALUE_MAX_LEN, description = "described value")]
    struct DescribedValue(
        bounded_types::bounded_string::BoundedString<0usize, { DESCRIBED_VALUE_MAX_LEN }, false>,
    );
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        PartialEq,
        Eq,
        proc_macro_newtype::BoundedStringWrapper,
    )]
    #[bounded_string(max = 3usize, min = constants_usize::ONE, chars, nul_free, serde, trim, utoipa)]
    struct RichValue(
        bounded_types::bounded_string::BoundedString<{ constants_usize::ONE }, 3usize, true>,
    );
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        PartialEq,
        Eq,
        proc_macro_newtype::BoundedStringWrapper,
    )]
    #[bounded_string(max = 3usize, chars, utoipa, write_only)]
    struct WriteOnlyValue(bounded_types::bounded_string::BoundedString<0usize, 3usize, true>);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        PartialEq,
        Eq,
        proc_macro_newtype::BoundedStringWrapper,
    )]
    #[bounded_string(max = 3usize, validator = VALIDATE_LOWERCASE_ASCII)]
    struct ValidatedValue(bounded_types::bounded_string::BoundedString<0usize, 3usize, false>);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        proc_macro_newtype::Display,
        proc_macro_newtype::FromInner,
        proc_macro_newtype::IntoInner,
        proc_macro_newtype::ToTokens,
    )]
    struct ProcMacro2TokenValue(proc_macro2::TokenStream);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq,
    )]
    struct ReferentValue<'value_lt>(&'value_lt str, &'value_lt str);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        Copy,
        proc_macro_newtype::AsRefInner,
        proc_macro_newtype::FromInner,
    )]
    struct ReferentValueRef<'value_lt>(&'value_lt ReferentValue<'value_lt>);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        PartialEq,
        Eq,
        proc_macro_newtype::AsRefOwned,
        proc_macro_newtype::FromInner,
        proc_macro_newtype::PartialEqInner,
    )]
    struct OwnedValue(Vec<u8>);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        PartialEq,
        Eq,
        proc_macro_newtype::AsRefTarget,
        proc_macro_newtype::FromInner,
    )]
    struct OwnedSliceValue(Vec<u8>);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        Copy,
        proc_macro_newtype::AsRefInner,
        proc_macro_newtype::BorrowInner,
        proc_macro_newtype::FromInner,
    )]
    struct SliceValueRef<'value_lt>(&'value_lt [u8]);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype::BorrowPath,
        proc_macro_newtype::FromInner,
    )]
    struct OwnedPathBuf(std::path::PathBuf);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype::DebugTransparent,
        proc_macro_newtype::FromInner,
    )]
    struct TransparentDebugValue(u16);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype::DebugRedacted,
        proc_macro_newtype::FromInner,
    )]
    struct RedactedDebugValue(Vec<u8>);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype::FromInner,
        proc_macro_newtype::NotInner,
    )]
    struct BoolValue(bool);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        proc_macro_newtype::DebugDisplay,
    )]
    enum DebugDisplayError {
        Failed,
    }
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::DisplayConst,
    )]
    #[display_const("fixed")]
    struct ConstDisplayError;
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype::CloneInner,
        proc_macro_newtype::FromInner,
    )]
    struct GenericValueArc<Value>(std::sync::Arc<Value>);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype::DefaultInner,
        proc_macro_newtype::FromInner,
    )]
    struct GenericVec<Value>(Vec<Value>);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype::AsMut,
        proc_macro_newtype::FromInner,
    )]
    struct MutableValueRef<'value_lt>(&'value_lt mut u16);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq,
    )]
    enum CheckedTextError {
        TooLong,
    }
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        PartialEq,
        Eq,
        proc_macro_newtype::TryFrom,
    )]
    #[try_from(validator = validate_checked_text)]
    struct CheckedText(String);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        PartialEq,
        Eq,
        proc_macro_newtype::TryFrom,
    )]
    #[try_from(error = CheckedTextError, validator = validate_checked_text)]
    struct ExplicitErrorCheckedText(String);
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        proc_macro_newtype::EnumFromStr,
    )]
    enum SampleEnum {
        FirstValue,
        Second,
    }
    fn validate_checked_text(value: &str) -> Result<(), CheckedTextError> {
        if value.len() > 2usize {
            Err(CheckedTextError::TooLong)
        } else {
            Ok(())
        }
    }
    #[test]
    fn test_not_inner_forwards_to_inner_value() {
        assert!(!BoolValue(false), "f2b418c7");
    }
    #[test]
    fn test_debug_display_formats_with_debug() {
        assert_eq!(DebugDisplayError::Failed.to_string(), "Failed", "a67e3b91");
    }
    #[test]
    fn test_display_const_formats_configured_expression() {
        assert_eq!(ConstDisplayError.to_string(), "fixed", "e5a9217c");
    }
    #[test]
    fn test_clone_and_default_inner_do_not_require_value_bounds() {
        #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
        struct NotCloneOrDefault;
        let vec_value = GenericVec::<NotCloneOrDefault>::default();
        assert!(vec_value.0.is_empty(), "cb741d96");
        let arc_value = GenericValueArc::from(std::sync::Arc::new(NotCloneOrDefault));
        let cloned = arc_value.clone();
        assert_eq!(std::sync::Arc::strong_count(&cloned.0), 2usize, "03c8e1f4");
        assert!(std::sync::Arc::ptr_eq(&arc_value.0, &cloned.0), "01da5e7c");
    }
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(dead_code)]
    fn dependency_markers(
        _: Option<proc_macro2::TokenStream>,
        _: Option<syn::Path>,
        _: Option<workspace_macro_helpers::first_identifier::FirstIdentifier>,
    ) -> proc_macro2::TokenStream {
        quote::quote! {}
    }
    #[test]
    fn test_string_newtype_impls_are_generated() {
        let v = StringValue::try_from(String::from(constants_str::ABC_ALT_3))
            .expect(constants_str::DIAGNOSTIC_9D27B01C);
        assert_eq!(v.to_string(), "abc");
        assert_eq!(v.as_ref(), "abc");
        assert_eq!(&*v, "abc");
        assert_eq!(StringValueProvider::string_value(&v), "abc");
        assert_eq!(
            to_err_string::to_err_string::ToErrString::to_err_string(&v).as_ref(),
            "abc"
        );
        assert_eq!(
            StringValue::try_from("abc".to_owned()).map(|value| value.to_string()),
            Ok(String::from("abc"))
        );
    }
    #[test]
    fn test_try_from_validator_generates_checked_conversion() {
        assert_eq!(
            CheckedText::try_from(constants_str::AB.to_owned()),
            Ok(CheckedText(constants_str::AB.to_owned()))
        );
        assert_eq!(
            CheckedText::try_from(constants_str::ABC_ALT_3.to_owned()),
            Err(CheckedTextError::TooLong)
        );
    }
    #[test]
    fn test_try_from_validator_supports_explicit_error_type() {
        assert_eq!(
            ExplicitErrorCheckedText::try_from(constants_str::AB.to_owned()),
            Ok(ExplicitErrorCheckedText(constants_str::AB.to_owned()))
        );
        assert_eq!(
            ExplicitErrorCheckedText::try_from(constants_str::ABC_ALT_3.to_owned()),
            Err(CheckedTextError::TooLong)
        );
    }
    #[test]
    fn test_display_to_err_string_impl_is_generated() {
        let v = UsizeValue::from(42usize);
        assert_eq!(v.to_string(), "42");
        assert_eq!(
            to_err_string::to_err_string::ToErrString::to_err_string(&v).as_ref(),
            "42"
        );
    }
    #[test]
    fn test_debug_to_err_string_impl_is_generated() {
        let v = DebugValue::from(vec![1, 2]);
        assert_eq!(
            to_err_string::to_err_string::ToErrString::to_err_string(&v).as_ref(),
            "[1, 2]"
        );
    }
    #[test]
    fn test_inner_accessors_are_generated() {
        let v = InnerValue::from(7);
        assert_eq!(*v.as_ref(), 7);
        assert_eq!(v.into_inner(), 7);
    }
    #[test]
    fn test_from_getter_converts_source_through_getter() {
        assert_eq!(
            FromGetterValue::from(GetterSource(429u16)),
            FromGetterValue(429u16)
        );
    }
    #[test]
    fn test_direct_inner_accessors_are_generated() {
        let text = GetInnerValueRef::from(constants_str::ABC_ALT_3).get();
        let flag = GetInnerBool::from(true).get();
        assert_eq!(text, constants_str::ABC_ALT_3);
        assert!(std::hint::black_box(flag));
    }
    #[test]
    fn test_vec_accessors_are_generated() {
        let v = VecValue::from(vec![1i32, 2i32]);
        assert_eq!(v.as_slice(), [1i32, 2i32]);
        assert_eq!(v.into_vec(), [1i32, 2i32]);
    }
    #[test]
    fn test_inner_deref_and_from_are_generated() {
        let mut v = InnerVecValue::from(vec![1u8]);
        v.push(2);
        assert_eq!(&*v, &[1, 2]);
        assert_eq!(Vec::<u8>::from(v), [1, 2]);
    }
    #[test]
    fn test_consuming_into_iterator_is_generated() {
        let value = InnerVecValue::from(vec![1u8, 2u8]);
        assert_eq!(value.into_iter().collect::<Vec<u8>>(), [1u8, 2u8]);
    }
    #[test]
    fn test_target_deref_impls_are_generated() {
        let mut v = TargetVecValue::from(Vec::from(*b"lower"));
        v.make_ascii_uppercase();
        assert_eq!(&*v, b"LOWER");
    }
    #[test]
    fn test_redacted_debug_does_not_expose_inner_value() {
        let value = RedactedDebugValue::from(constants_str::SECRET.as_bytes().to_vec());
        let output = format!("{value:?}");
        assert!(output.contains(constants_str::REDACTED_ALT_3));
        assert!(!output.contains(constants_str::SECRET));
        assert_eq!(value.0, constants_str::SECRET.as_bytes());
    }
    #[test]
    fn test_mutable_reference_as_mut_is_generated() {
        let mut inner = 1u16;
        let mut value = MutableValueRef::from(&mut inner);
        *AsMut::<u16>::as_mut(&mut value) = 2u16;
        assert_eq!(inner, 2u16);
    }
    #[test]
    fn test_bounded_string_description_is_configurable() {
        let error = DescribedValue::try_from(String::from(constants_str::ABC_ALT_3))
            .expect_err(constants_str::VALUE_3DFCA278);
        assert_eq!(
            error.to_string(),
            "described value length 3 exceeds maximum 2"
        );
    }
    #[test]
    fn test_bounded_string_rich_policies_share_runtime_and_serde_validation() {
        let bounded_value = [
            ' ',
            ' ',
            char::from_u32(0x430).expect(constants_str::DIAGNOSTIC_A4FC1902),
            char::from_u32(0x431).expect(constants_str::DIAGNOSTIC_B9E21C73),
            ' ',
            ' ',
        ]
        .into_iter()
        .collect::<String>();
        let expected = bounded_value.trim().to_owned();
        let value =
            RichValue::try_from(bounded_value.clone()).expect(constants_str::DIAGNOSTIC_A091B772);
        assert_eq!(value.0.as_ref(), &expected);
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
        let serialized =
            serde_json::to_string(&bounded_value).expect(constants_str::DIAGNOSTIC_7C3A9D21);
        let deserialized = serde_json::from_str::<RichValue>(&serialized)
            .expect(constants_str::DIAGNOSTIC_1D3222B1);
        assert_eq!(deserialized.0.as_ref(), &expected);
        let _error = serde_json::from_str::<RichValue>(constants_str::ABCD)
            .expect_err(constants_str::C0E03C6D);
    }
    #[test]
    fn test_bounded_string_openapi_limits_match_runtime_limits() {
        let schema = <RichValue as utoipa::PartialSchema>::schema();
        let json = serde_json::to_value(schema).expect(constants_str::DIAGNOSTIC_756F3FE9);
        assert_eq!(
            json.get("minLength"),
            Some(&serde_json::json!(constants_usize::ONE))
        );
        assert_eq!(json.get("maxLength"), Some(&serde_json::json!(3usize)));
    }
    #[test]
    fn test_bounded_string_openapi_write_only_matches_secret_contract() {
        let schema = <WriteOnlyValue as utoipa::PartialSchema>::schema();
        let json = serde_json::to_value(schema).expect(constants_str::DIAGNOSTIC_CE9351D4);
        assert_eq!(json.get("writeOnly"), Some(&serde_json::json!(true)));
    }
    #[test]
    fn test_utoipa_schema_delegates_default_and_overridden_types() {
        let delegated_value =
            UtoipaSchemaDelegatedValue::from(bounded_types::bounded_vec::BoundedVec::default());
        let overridden_value = UtoipaSchemaOverrideValue::from(Vec::new());
        let UtoipaSchemaDelegatedValue(delegated_inner) = delegated_value;
        let UtoipaSchemaOverrideValue(overridden_inner) = overridden_value;
        assert!(delegated_inner.is_empty());
        assert!(overridden_inner.is_empty());
        let expected = serde_json::to_value(<bounded_types::bounded_vec::BoundedVec<
            u8,
            0,
            { constants_usize::TWO },
        > as utoipa::PartialSchema>::schema())
        .expect(constants_str::DIAGNOSTIC_38274C1A);
        let delegated =
            serde_json::to_value(<UtoipaSchemaDelegatedValue as utoipa::PartialSchema>::schema())
                .expect(constants_str::DIAGNOSTIC_5DA321B7);
        let overridden =
            serde_json::to_value(<UtoipaSchemaOverrideValue as utoipa::PartialSchema>::schema())
                .expect(constants_str::DIAGNOSTIC_2B3C6772);
        assert_eq!(delegated, expected);
        assert_eq!(overridden, expected);
    }
    #[test]
    fn test_bounded_string_small_input_space_matches_reference_model() {
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
                    && (constants_usize::ONE..=3usize).contains(&normalized.chars().count());
                RichValue::try_from(value).is_ok() == expected_ok
            });
        assert!(all_match);
    }
    #[test]
    fn test_bounded_string_custom_validator_is_applied() {
        let value = ValidatedValue::try_from(String::from(constants_str::ABC_ALT_3))
            .expect(constants_str::DIAGNOSTIC_FCADF793);
        assert_eq!(value.0.as_ref(), constants_str::ABC_ALT_3);
        assert!(matches!(
            ValidatedValue::try_from(String::from(constants_str::GET)),
            Err(ValidatedValueTryFromStringError::InvalidValue)
        ));
    }
    #[test]
    fn test_token_impls_are_generated() {
        let inner = quote::quote! { sample::path };
        let v = ProcMacro2TokenValue::from(inner.clone());
        assert_eq!(v.to_string(), "sample :: path");
        assert_eq!(quote::quote! { #v }.to_string(), inner.to_string());
        assert_eq!(v.into_inner().to_string(), inner.to_string());
    }
    #[test]
    fn test_reference_inner_impls_are_generated() {
        let inner = ReferentValue(constants_str::LEFT, constants_str::RIGHT);
        let v = ReferentValueRef::from(&inner);
        assert_eq!(AsRef::<ReferentValue<'_>>::as_ref(&v), &inner);
    }
    #[test]
    fn test_borrow_impls_are_generated() {
        let string = StringValue::try_from(String::from(constants_str::ABC_ALT_3))
            .expect(constants_str::DIAGNOSTIC_F37F2ED0);
        assert_eq!(std::borrow::Borrow::<str>::borrow(&string), "abc");
        let owned = InnerValue::from(7u16);
        assert_eq!(*std::borrow::Borrow::<u16>::borrow(&owned), 7u16);
        let path = OwnedPathBuf::from(std::path::PathBuf::from(constants_str::ABC_ALT_3));
        assert_eq!(
            std::borrow::Borrow::<std::path::Path>::borrow(&path),
            std::path::Path::new(constants_str::ABC_ALT_3)
        );
        let inner = SliceValueRef::from(constants_str::ABC_ALT_3.as_bytes());
        assert_eq!(std::borrow::Borrow::<[u8]>::borrow(&inner), b"abc");
    }
    #[test]
    fn test_owned_inner_impls_are_generated() {
        let v = OwnedValue::from(vec![3, 5, 8]);
        assert_eq!(v, vec![3, 5, 8], "72a4dc19");
        assert_eq!(AsRef::<Vec<u8>>::as_ref(&v), &[3, 5, 8]);
    }
    #[test]
    fn test_owned_and_borrowed_slice_impls_are_generated() {
        let bytes = vec![3u8, 5u8, 8u8];
        let owned = OwnedSliceValue::from(bytes.clone());
        let borrowed = SliceValueRef::from(bytes.as_slice());
        assert_eq!(AsRef::<[u8]>::as_ref(&owned), bytes.as_slice());
        assert_eq!(AsRef::<[u8]>::as_ref(&borrowed), bytes.as_slice());
    }
    #[test]
    fn test_transparent_debug_forwards_inner_format() {
        assert_eq!(format!("{:?}", TransparentDebugValue(17)), "17");
    }
    #[test]
    fn test_enum_from_str_impl_is_generated() {
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
    fn test_enum_from_str_error_mentions_allowed_values() {
        let error = <SampleEnum as std::str::FromStr>::from_str(constants_str::BAD)
            .expect_err(constants_str::VALUE_42D13F7A);
        assert_eq!(
            error,
            "Unknown value: bad. Allowed values: first_value, second"
        );
    }
}
