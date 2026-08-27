use super::{DefaultRegexPattern, RegexError, RegexRegexTryFromStringError};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefOwned,
    newtype::Display,
    newtype::IntoInnerFrom,
)]
#[serde(try_from = "String", into = "String")]
pub struct RegexRegex(pub(super) String);
impl utoipa::PartialSchema for RegexRegex {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .into()
    }
}
impl utoipa::ToSchema for RegexRegex {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::PG_CRUD_REGEX_REGEX_SCHEMA_NAME)
    }
}
impl TryFrom<String> for RegexRegex {
    type Error = RegexRegexTryFromStringError;
    fn try_from(v: String) -> Result<Self, Self::Error> {
        if v.len() > constants_usize::VALUE_1_048_576 {
            return Err(RegexRegexTryFromStringError::TooLong);
        }
        let _validated_regex = regex::Regex::new(&v).map_err(RegexError::from)?;
        Ok(Self(v))
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
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
        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
            { generator.subschema_for::<String>() }
        }
        fn inline_schema() -> bool {
            false
        }
    }
};
impl pg_crud_common::domain_types::DefaultSomeOneElement for RegexRegex {
    fn default_some_one_element() -> Self {
        Self::from(DefaultRegexPattern)
    }
}
