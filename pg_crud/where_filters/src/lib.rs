generate_where_filters::generate_where_filters!({
    "pg_types_write_into_file": "False",
    "whole_write_into_file": "False"
});
const REGEX_PATTERN_MAX_BYTES: usize = 1_048_576usize;
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    utoipa::ToSchema,
    optml::Optml,
)]
pub enum EncodeFormat {
    #[default]
    Base64,
    Escape,
    Hex,
}
impl std::fmt::Display for EncodeFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Base64 => write!(f, "base64"),
            Self::Escape => write!(f, "escape"),
            Self::Hex => write!(f, "hex"),
        }
    }
}
impl pg_crud_common::DefaultSomeOneElement for EncodeFormat {
    fn default_some_one_element() -> Self {
        Self::default()
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    optml::Optml,
    newtype::AsRefOwned,
    newtype::Display,
    newtype::IntoInnerFrom,
)]
#[serde(try_from = "String", into = "String")]
pub struct RegexRegex(String);
#[derive(Clone, Copy, Debug)]
struct DefaultRegexPattern;
impl From<DefaultRegexPattern> for RegexRegex {
    fn from(_value: DefaultRegexPattern) -> Self {
        Self(String::from(str_constants::A_Z_PLUS))
    }
}
#[derive(Debug, newtype::ErrorTransparent, newtype::FromInner, newtype::Display)]
pub struct RegexError(regex::Error);
#[derive(Debug, thiserror::Error)]
pub enum RegexRegexTryFromStringError {
    #[error("regular expression pattern is invalid")]
    Regex(#[from] RegexError),
    #[error("regular expression pattern exceeds the size limit")]
    TooLong,
}
impl<'schema_lt> utoipa::ToSchema<'schema_lt> for RegexRegex {
    fn schema() -> (
        &'schema_lt str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            str_constants::PG_CRUD_REGEX_REGEX_SCHEMA_NAME,
            utoipa::openapi::ObjectBuilder::new()
                .schema_type(utoipa::openapi::SchemaType::String)
                .into(),
        )
    }
}
impl TryFrom<String> for RegexRegex {
    type Error = RegexRegexTryFromStringError;
    fn try_from(v: String) -> Result<Self, Self::Error> {
        if v.len() > REGEX_PATTERN_MAX_BYTES {
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
                str_constants::PG_CRUD_REGEX_REGEX_SCHEMA_NAME,
            )
        }
        fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
            schemars::_private::alloc::borrow::Cow::Borrowed(
                str_constants::PG_CRUD_REGEX_REGEX_SCHEMA_ID,
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
impl pg_crud_common::DefaultSomeOneElement for RegexRegex {
    fn default_some_one_element() -> Self {
        Self::from(DefaultRegexPattern)
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    utoipa::ToSchema,
    optml::Optml,
)]
pub enum RegexCase {
    Insensitive,
    Sensitive,
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optml::Optml,
    newtype::AsRefInner,
    newtype::Display,
    newtype::FromInner,
)]
pub struct RegexCasePostgreqlSyntax(&'static str);
impl pg_crud_common::DefaultSomeOneElement for RegexCase {
    fn default_some_one_element() -> Self {
        Self::Sensitive
    }
}
impl RegexCase {
    #[must_use]
    pub fn postgreql_syntax(&self) -> RegexCasePostgreqlSyntax {
        match &self {
            Self::Insensitive => RegexCasePostgreqlSyntax::from(str_constants::ASTERISK_ALT),
            Self::Sensitive => RegexCasePostgreqlSyntax::from(str_constants::TEXT_ALT_15),
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, schemars::JsonSchema, optml::Optml)]
pub struct Between<T>
where
    T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
{
    start: T,
    end: T,
}
impl<'schema_lt, T> utoipa::ToSchema<'schema_lt> for Between<T>
where
    T: sqlx::Type<sqlx::Postgres>
        + for<'encode_lt> sqlx::Encode<'encode_lt, sqlx::Postgres>
        + utoipa::ToSchema<'schema_lt>,
{
    fn schema() -> (
        &'schema_lt str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            str_constants::PG_CRUD_BETWEEN_SCHEMA_NAME,
            utoipa::openapi::ObjectBuilder::new()
                .property(
                    str_constants::PG_CRUD_START_FIELD,
                    <T as utoipa::ToSchema>::schema().1,
                )
                .property(
                    str_constants::PG_CRUD_END_FIELD,
                    <T as utoipa::ToSchema>::schema().1,
                )
                .required(str_constants::PG_CRUD_START_FIELD)
                .required(str_constants::PG_CRUD_END_FIELD)
                .build()
                .into(),
        )
    }
}
#[location::errors_with_location]
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    optml::Optml,
)]
pub enum BetweenTryNewError<T> {
    StartMoreOrEqToEnd {
        #[eo_to_err_string_serde]
        start: T,
        #[eo_to_err_string_serde]
        end: T,
    },
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + PartialOrd>
    Between<T>
{
    pub fn try_new(start: T, end: T) -> Result<Self, BetweenTryNewError<T>> {
        if start < end {
            Ok(Self { start, end })
        } else {
            Err(BetweenTryNewError::StartMoreOrEqToEnd {
                start,
                end,
                location: location_macros::location!(),
            })
        }
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T> _serde::Deserialize<'de> for Between<T>
    where
        T: std::fmt::Debug
            + _serde::Deserialize<'de>
            + PartialOrd
            + sqlx::Type<sqlx::Postgres>
            + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[expect(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                f0,
                f1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __f: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __f,
                        str_constants::PG_CRUD_FIELD_IDENTIFIER,
                    )
                }
                fn visit_u64<__E>(self, v: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match v {
                        1u64 => Ok(__Field::f0),
                        2u64 => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, v: &str) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match v {
                        str_constants::PG_CRUD_START_FIELD => Ok(__Field::f0),
                        str_constants::PG_CRUD_END_FIELD => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, v: &[u8]) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match v {
                        b"start" => Ok(__Field::f0),
                        b"end" => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>
                    + sqlx::Type<sqlx::Postgres>
                    + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
            {
                marker: _serde::__private228::PhantomData<Between<T>>,
                lt: _serde::__private228::PhantomData<&'de ()>,
            }
            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
            where
                T: std::fmt::Debug
                    + _serde::Deserialize<'de>
                    + PartialOrd
                    + sqlx::Type<sqlx::Postgres>
                    + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
            {
                type Value = Between<T>;
                fn expecting(
                    &self,
                    __f: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __f,
                        str_constants::PG_CRUD_BETWEEN_STRUCT_NAME,
                    )
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(f0) = _serde::de::SeqAccess::next_element::<T>(&mut __seq)? else {
                        return Err(_serde::de::Error::invalid_length(
                            1usize,
                            &str_constants::PG_CRUD_BETWEEN_EXPECTING,
                        ));
                    };
                    let Some(f1) = _serde::de::SeqAccess::next_element::<T>(&mut __seq)? else {
                        return Err(_serde::de::Error::invalid_length(
                            2usize,
                            &str_constants::PG_CRUD_BETWEEN_EXPECTING,
                        ));
                    };
                    match Between::try_new(f0, f1) {
                        Ok(v) => Ok(v),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut f0: Option<T> = None;
                    let mut f1: Option<T> = None;
                    while let Some(__k) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __k {
                            __Field::f0 => {
                                if Option::is_some(&f0) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            str_constants::PG_CRUD_START_FIELD,
                                        ),
                                    );
                                }
                                f0 = Some(_serde::de::MapAccess::next_value::<T>(&mut __map)?);
                            }
                            __Field::f1 => {
                                if Option::is_some(&f1) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            str_constants::PG_CRUD_END_FIELD,
                                        ),
                                    );
                                }
                                f1 = Some(_serde::de::MapAccess::next_value::<T>(&mut __map)?);
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny =
                                    _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                        &mut __map,
                                    )?;
                            }
                        }
                    }
                    let f0_v = match f0 {
                        Some(v) => v,
                        None => _serde::__private228::de::missing_field(
                            str_constants::PG_CRUD_START_FIELD,
                        )?,
                    };
                    let f1_v = match f1 {
                        Some(v) => v,
                        None => _serde::__private228::de::missing_field(
                            str_constants::PG_CRUD_END_FIELD,
                        )?,
                    };
                    match Between::try_new(f0_v, f1_v) {
                        Ok(v) => Ok(v),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                str_constants::PG_CRUD_BETWEEN_SCHEMA_NAME,
                str_constants::PG_CRUD_SERDE_BETWEEN_FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lt: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<
    T: pg_crud_common::DefaultSomeOneElement
        + sqlx::Type<sqlx::Postgres>
        + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
> pg_crud_common::DefaultSomeOneElement for Between<T>
{
    fn default_some_one_element() -> Self {
        Self {
            start: pg_crud_common::DefaultSomeOneElement::default_some_one_element(),
            end: pg_crud_common::DefaultSomeOneElement::default_some_one_element(),
        }
    }
}
impl<'lt, T: Send + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + 'lt>
    pg_crud_common::PgTypeWhereFilter<'lt> for Between<T>
{
    fn query_bind(
        self,
        mut query: pg_crud_common::SqlxPostgresQuery<'lt>,
    ) -> Result<pg_crud_common::SqlxPostgresQuery<'lt>, pg_crud_common::SqlxPostgresQueryBindError>
    {
        if let Err(error) = query.as_mut().try_bind(self.start) {
            return Err(
                match pg_crud_common::SqlxPostgresQueryBindError::try_from(error.to_string()) {
                    Ok(v) => v,
                    Err(bind_error) => pg_crud_common::SqlxPostgresQueryBindError::from(bind_error),
                },
            );
        }
        if let Err(error) = query.as_mut().try_bind(self.end) {
            return Err(
                match pg_crud_common::SqlxPostgresQueryBindError::try_from(error.to_string()) {
                    Ok(v) => v,
                    Err(bind_error) => pg_crud_common::SqlxPostgresQueryBindError::from(bind_error),
                },
            );
        }
        Ok(query)
    }
    fn query_part(
        &self,
        increment: &mut dyn pg_crud_common::QueryPartIncrementMut,
        _: pg_crud_common::SqlColumnRef<'_>,
        _: pg_crud_common::AddOperator,
    ) -> Result<pg_crud_common::QueryPartFragment, pg_crud_common::QueryPartError> {
        let start_increment =
            match pg_crud_common::increment_checked_add_one_returning_increment(increment) {
                Ok(v) => v,
                Err(error) => {
                    return Err(error);
                }
            };
        let end_increment =
            match pg_crud_common::increment_checked_add_one_returning_increment(increment) {
                Ok(v) => v,
                Err(error) => {
                    return Err(error);
                }
            };
        let mut query_part = String::with_capacity(32);
        if std::fmt::Write::write_fmt(
            &mut query_part,
            format_args!("between ${start_increment} and ${end_increment}"),
        )
        .is_err()
        {
            return Err(pg_crud_common::QueryPartError::WriteIntoBuffer {
                location: location_macros::location!(),
            });
        }
        Ok(pg_crud_common::QueryPartFragment::try_from(query_part)?)
    }
}
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    schemars::JsonSchema,
    optml::Optml,
    newtype::AsSlice,
    newtype::IntoInnerFrom,
)]
pub struct PgTypeNotEmptyUniqueVec<T>(Vec<T>);
impl<T> From<[T; 1]> for PgTypeNotEmptyUniqueVec<T> {
    fn from(value: [T; 1]) -> Self {
        Self(Vec::from(value))
    }
}
impl<'schema_lt, T: utoipa::ToSchema<'schema_lt>> utoipa::ToSchema<'schema_lt>
    for PgTypeNotEmptyUniqueVec<T>
{
    fn schema() -> (
        &'schema_lt str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            str_constants::PG_CRUD_PG_TYPE_NOT_EMPTY_UNIQUE_VEC_SCHEMA_NAME,
            utoipa::openapi::ArrayBuilder::new()
                .items(<T as utoipa::ToSchema>::schema().1)
                .min_items(Some(1))
                .build()
                .into(),
        )
    }
}
impl<T: PartialEq> TryFrom<Vec<T>> for PgTypeNotEmptyUniqueVec<T> {
    type Error = pg_crud_common::NotEmptyUniqueVecTryNewError<T>;
    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        pg_crud_common::NotEmptyUniqueVec::try_new(v.into())
            .map(pg_crud_common::NotEmptyUniqueVec::into_vec)
            .map(Self)
    }
}
impl<T: Eq + std::hash::Hash> PgTypeNotEmptyUniqueVec<T> {
    pub fn try_from_by_hash(
        v: pg_crud_common::DuplicateCandidates<T>,
    ) -> Result<Self, pg_crud_common::NotEmptyUniqueVecTryNewError<T>> {
        pg_crud_common::NotEmptyUniqueVec::try_new_by_hash(v)
            .map(pg_crud_common::NotEmptyUniqueVec::into_vec)
            .map(Self)
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    #[expect(clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + PartialEq + _serde::Deserialize<'de>> _serde::Deserialize<'de>
        for PgTypeNotEmptyUniqueVec<T>
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private228::PhantomData<PgTypeNotEmptyUniqueVec<T>>,
                lt: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T: std::fmt::Debug + PartialEq + _serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = PgTypeNotEmptyUniqueVec<T>;
                fn expecting(
                    &self,
                    __f: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __f,
                        str_constants::PG_CRUD_PG_TYPE_NOT_EMPTY_UNIQUE_VEC_TUPLE_NAME,
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let f0: Vec<T> = <Vec<T> as _serde::Deserialize>::deserialize(__e)?;
                    match PgTypeNotEmptyUniqueVec::try_from(f0) {
                        Ok(v) => Ok(v),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(f0) = _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &str_constants::PG_CRUD_PG_TYPE_NOT_EMPTY_UNIQUE_VEC_TUPLE_EXPECTING,
                        ));
                    };
                    match PgTypeNotEmptyUniqueVec::try_from(f0) {
                        Ok(v) => Ok(v),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                str_constants::PG_CRUD_PG_TYPE_NOT_EMPTY_UNIQUE_VEC_SCHEMA_NAME,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lt: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<T: pg_crud_common::DefaultSomeOneElement> pg_crud_common::DefaultSomeOneElement
    for PgTypeNotEmptyUniqueVec<T>
{
    fn default_some_one_element() -> Self {
        Self::from([pg_crud_common::DefaultSomeOneElement::default_some_one_element()])
    }
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optml::Optml,
    newtype::AsSlice,
    newtype::IntoInner,
)]
#[serde(try_from = "Vec<T>")]
pub struct BoundedVec<T, const LENGTH: usize>(Vec<T>);
impl<T, const LENGTH: usize> From<[T; LENGTH]> for BoundedVec<T, LENGTH> {
    fn from(value: [T; LENGTH]) -> Self {
        Self(Vec::from(value))
    }
}
#[location::errors_with_location]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    schemars::JsonSchema,
    optml::Optml,
)]
pub enum BoundedVecTryNewError {
    LenIsNotCorrect {
        #[eo_to_err_string_serde]
        wrong_len: BoundedVecLen,
        #[eo_to_err_string_serde]
        expected: BoundedVecLen,
    },
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optml::Optml,
    newtype::Display,
    newtype::FromInner,
)]
#[serde(from = "usize")]
pub struct BoundedVecLen(usize);
impl BoundedVecLen {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}
impl to_err_string::ToErrString for BoundedVecLen {
    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
        to_err_string::ToErrStringValue::try_from(self.to_string())
            .unwrap_or_else(to_err_string::ToErrStringValue::from)
    }
}
enum Variant {
    MinusOne,
    Normal,
}
impl<
    'lt,
    T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + 'lt,
    const LENGTH: usize,
> BoundedVec<T, LENGTH>
{
    pub fn pg_type_query_part(
        &self,
        increment: &mut dyn pg_crud_common::QueryPartIncrementMut,
        column: pg_crud_common::SqlColumnRef<'_>,
        add_operator: pg_crud_common::AddOperator,
    ) -> Result<pg_crud_common::QueryPartFragment, pg_crud_common::QueryPartError> {
        self.query_part(increment, column, add_operator, &Variant::Normal)
    }
    pub fn pg_type_query_part_minus_one(
        &self,
        increment: &mut dyn pg_crud_common::QueryPartIncrementMut,
        column: pg_crud_common::SqlColumnRef<'_>,
        add_operator: pg_crud_common::AddOperator,
    ) -> Result<pg_crud_common::QueryPartFragment, pg_crud_common::QueryPartError> {
        self.query_part(increment, column, add_operator, &Variant::MinusOne)
    }
    pub fn query_bind(
        self,
        query: pg_crud_common::SqlxPostgresQuery<'lt>,
    ) -> Result<pg_crud_common::SqlxPostgresQuery<'lt>, pg_crud_common::SqlxPostgresQueryBindError>
    {
        self.0
            .into_iter()
            .try_fold(query, |mut accumulator_query, element| {
                accumulator_query
                    .as_mut()
                    .try_bind(element)
                    .map_err(
                        |error| match pg_crud_common::SqlxPostgresQueryBindError::try_from(
                            error.to_string(),
                        ) {
                            Ok(v) => v,
                            Err(bind_error) => {
                                pg_crud_common::SqlxPostgresQueryBindError::from(bind_error)
                            }
                        },
                    )?;
                Ok(accumulator_query)
            })
    }
    fn query_part(
        &self,
        increment: &mut dyn pg_crud_common::QueryPartIncrementMut,
        _: pg_crud_common::SqlColumnRef<'_>,
        _add_operator: pg_crud_common::AddOperator,
        variant: &Variant,
    ) -> Result<pg_crud_common::QueryPartFragment, pg_crud_common::QueryPartError> {
        let len = match &variant {
            Variant::MinusOne => self.0.len().saturating_sub(1),
            Variant::Normal => self.0.len(),
        };
        let mut accumulator = String::with_capacity(len.saturating_mul(8));
        (0..len).try_for_each(|_| {
            let v = match pg_crud_common::increment_checked_add_one_returning_increment(increment) {
                Ok(v) => v,
                Err(error) => {
                    return Err(error);
                }
            };
            let write_res = std::fmt::Write::write_fmt(&mut accumulator, format_args!("[${v}]"));
            if write_res.is_err() {
                return Err(pg_crud_common::QueryPartError::WriteIntoBuffer {
                    location: location_macros::location!(),
                });
            }
            Ok::<(), pg_crud_common::QueryPartError>(())
        })?;
        Ok(pg_crud_common::QueryPartFragment::try_from(accumulator)?)
    }
}
impl<T, const LENGTH: usize> TryFrom<Vec<T>> for BoundedVec<T, LENGTH> {
    type Error = BoundedVecTryNewError;
    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        let len = v.len();
        if len == LENGTH {
            Ok(Self(v))
        } else {
            Err(BoundedVecTryNewError::LenIsNotCorrect {
                wrong_len: BoundedVecLen::from(len),
                expected: BoundedVecLen::from(LENGTH),
                location: location_macros::location!(),
            })
        }
    }
}
impl<T: Clone + pg_crud_common::DefaultSomeOneElement, const LENGTH: usize>
    pg_crud_common::DefaultSomeOneElement for BoundedVec<T, LENGTH>
{
    fn default_some_one_element() -> Self {
        Self::from(std::array::from_fn(|_| {
            <T as pg_crud_common::DefaultSomeOneElement>::default_some_one_element()
        }))
    }
}
#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq, Eq, newtype::FromInner)]
    struct NonClone(u8);
    #[test]
    fn pg_type_not_empty_unique_vec_try_from_ok() {
        let rslt = super::PgTypeNotEmptyUniqueVec::<i32>::try_from(vec![1i32, 2i32, 3i32]);
        if let Err(error) = rslt {
            panic!("5a6afcfa {error:?}");
        }
    }
    #[test]
    fn pg_type_not_empty_unique_vec_try_from_empty() {
        let rslt = super::PgTypeNotEmptyUniqueVec::<i32>::try_from(Vec::new());
        assert!(matches!(
            rslt,
            Err(pg_crud_common::NotEmptyUniqueVecTryNewError::IsEmpty { .. })
        ));
    }
    #[test]
    fn pg_type_not_empty_unique_vec_try_from_not_unique() {
        let rslt = super::PgTypeNotEmptyUniqueVec::<i32>::try_from(vec![1i32, 2i32, 1i32]);
        assert!(matches!(
            rslt,
            Err(pg_crud_common::NotEmptyUniqueVecTryNewError::NotUnique { v: 1i32, .. })
        ));
    }
    #[test]
    fn pg_type_not_empty_unique_vec_try_from_too_long() {
        let rslt = super::PgTypeNotEmptyUniqueVec::<usize>::try_from(
            (0usize..=10_000usize).collect::<Vec<_>>(),
        );
        assert!(matches!(
            rslt,
            Err(pg_crud_common::NotEmptyUniqueVecTryNewError::TooLong { .. })
        ));
    }
    #[test]
    fn pg_type_not_empty_unique_vec_try_from_by_hash_not_unique() {
        let rslt =
            super::PgTypeNotEmptyUniqueVec::<i32>::try_from_by_hash(vec![1i32, 2i32, 1i32].into());
        assert!(matches!(
            rslt,
            Err(pg_crud_common::NotEmptyUniqueVecTryNewError::NotUnique { v: 1i32, .. })
        ));
    }
    #[test]
    fn pg_type_not_empty_unique_vec_try_from_supports_non_clone_values() {
        let rslt = super::PgTypeNotEmptyUniqueVec::<NonClone>::try_from(vec![
            NonClone(1),
            NonClone(2),
            NonClone(1),
        ]);
        assert!(matches!(
            rslt,
            Err(pg_crud_common::NotEmptyUniqueVecTryNewError::NotUnique { v: NonClone(1), .. })
        ));
    }
    #[test]
    fn encode_format_display_is_stable() {
        assert_eq!(super::EncodeFormat::Base64.to_string(), "base64");
        assert_eq!(super::EncodeFormat::Escape.to_string(), "escape");
        assert_eq!(super::EncodeFormat::Hex.to_string(), "hex");
    }
    #[test]
    fn regex_regex_eq_compares_pattern_content() {
        let left =
            super::RegexRegex::try_from(String::from(str_constants::D_PLUS)).expect("8342ad27");
        let right =
            super::RegexRegex::try_from(String::from(str_constants::D_PLUS)).expect("4d0fa8e3");
        let other =
            super::RegexRegex::try_from(String::from(str_constants::A_Z_PLUS)).expect("abcc9a72");
        assert_eq!(left, right);
        assert_ne!(left, other);
    }
}
