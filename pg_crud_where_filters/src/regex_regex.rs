#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::Display,
    proc_macro_newtype::IntoInnerFrom,
)]
#[serde(try_from = "String", into = "String")]
#[schema(value_type = String)]
pub struct RegexRegex(String);
impl From<crate::default_regex_pattern::DefaultRegexPattern> for RegexRegex {
    fn from(default_regex_pattern: crate::default_regex_pattern::DefaultRegexPattern) -> Self {
        let _: crate::default_regex_pattern::DefaultRegexPattern = default_regex_pattern;
        Self(String::from(constants_str::A_Z_PLUS))
    }
}
impl TryFrom<String> for RegexRegex {
    type Error = crate::regex_regex_try_from_string_error::RegexRegexTryFromStringError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.len() > constants_usize::VALUE_1_048_576 {
            return Err(
                crate::regex_regex_try_from_string_error::RegexRegexTryFromStringError::TooLong,
            );
        }
        let _validated_regex =
            regex::Regex::new(&string).map_err(crate::regex_error::RegexError::from)?;
        Ok(Self(string))
    }
}

#[allow(unused_qualifications, reason = "lint suppression is required here")]
#[allow(clippy::absolute_paths, reason = "lint suppression is required here")]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "lint suppression is required here"
)]
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces, reason = "lint suppression is required here")]
    impl schemars::JsonSchema for RegexRegex {
        fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
            schemars::_private::alloc::borrow::Cow::Borrowed(
                constants_str::PG_CRUD_REGEX_REGEX_SCHEMA_NAME,
            )
        }
        fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
            schemars::_private::alloc::borrow::Cow::Borrowed(
                constants_str::PG_CRUD_REGEX_REGEX_SCHEMA_ID,
            )
        }
        fn json_schema(schema_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
            { schema_generator.subschema_for::<String>() }
        }
        fn inline_schema() -> bool {
            false
        }
    }
};
impl pg_crud_common::default_some_one_element::DefaultSomeOneElement for RegexRegex {
    fn default_some_one_element() -> Self {
        Self::from(crate::default_regex_pattern::DefaultRegexPattern)
    }
}
