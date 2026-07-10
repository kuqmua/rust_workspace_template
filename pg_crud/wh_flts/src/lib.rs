gen_wh_flts::gen_wh_flts!({
    "pg_types_write_into_file": "False",
    "whole_write_into_file": "False"
});
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
impl pg_crud_cmn::DfltSomeOneEl for EncodeFormat {
    fn dflt_some_one_el() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, optml::Optml)]
#[serde(try_from = "String", into = "String")]
pub struct RegexRgx(regex::Regex);
impl From<regex::Regex> for RegexRgx {
    fn from(value: regex::Regex) -> Self {
        Self(value)
    }
}
impl AsRef<regex::Regex> for RegexRgx {
    fn as_ref(&self) -> &regex::Regex {
        &self.0
    }
}
impl TryFrom<String> for RegexRgx {
    type Error = regex::Error;
    fn try_from(v: String) -> Result<Self, Self::Error> {
        regex::Regex::new(&v).map(Self::from)
    }
}
impl From<RegexRgx> for String {
    fn from(v: RegexRgx) -> Self {
        v.0.as_str().to_owned()
    }
}
// #[automatically_derived]
// impl ::core::marker::StructuralPartialEq for RegexRegex {}
// #[automatically_derived]
impl PartialEq for RegexRgx {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}
//todo add some logic? for regex validation?
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for RegexRgx {
        fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
            schemars::_private::alloc::borrow::Cow::Borrowed("RegexRegex")
        }
        fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
            schemars::_private::alloc::borrow::Cow::Borrowed("tests::RegexRegex")
        }
        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
            { generator.subschema_for::<String>() }
        }
        fn inline_schema() -> bool {
            false
        }
    }
};
impl std::fmt::Display for RegexRgx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl pg_crud_cmn::DfltSomeOneEl for RegexRgx {
    fn dflt_some_one_el() -> Self {
        match regex::Regex::new("[a-z]+") {
            Ok(v) => Self(v),
            Err(er) => {
                eprintln!("22a9eda5: {er}");
                std::process::abort();
            }
        }
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
    optml::Optml,
)]
pub enum RgxCase {
    Insensitive,
    Sensitive,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, optml::Optml)]
pub struct RgxCasePostgreqlSyntax(&'static str);
impl From<&'static str> for RgxCasePostgreqlSyntax {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}
impl AsRef<str> for RgxCasePostgreqlSyntax {
    fn as_ref(&self) -> &str {
        self.0
    }
}
impl std::fmt::Display for RgxCasePostgreqlSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
impl pg_crud_cmn::DfltSomeOneEl for RgxCase {
    fn dflt_some_one_el() -> Self {
        Self::Sensitive
    }
}
impl RgxCase {
    #[must_use]
    pub fn postgreql_syntax(&self) -> RgxCasePostgreqlSyntax {
        match &self {
            Self::Insensitive => RgxCasePostgreqlSyntax::from("~*"),
            Self::Sensitive => RgxCasePostgreqlSyntax::from("~"),
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, schemars::JsonSchema, optml::Optml)]
pub struct Btwn<T>
where
    T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
{
    start: T,
    end: T,
}
#[location::errors_with_loc]
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    optml::Optml,
)]
pub enum BtwnTryNewEr<T> {
    StartMoreOrEqToEnd {
        #[eo_to_err_string_serde]
        start: T,
        #[eo_to_err_string_serde]
        end: T,
    },
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + PartialOrd>
    Btwn<T>
{
    pub fn try_new(start: T, end: T) -> Result<Self, BtwnTryNewEr<T>> {
        if start < end {
            Ok(Self { start, end })
        } else {
            Err(BtwnTryNewEr::StartMoreOrEqToEnd {
                start,
                end,
                loc: loc_macros::loc!(),
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
    impl<'de, T> _serde::Deserialize<'de> for Btwn<T>
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
                    _serde::__private228::Formatter::write_str(__f, "field identifier")
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
                        "start" => Ok(__Field::f0),
                        "end" => Ok(__Field::f1),
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
                marker: _serde::__private228::PhantomData<Btwn<T>>,
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
                type Value = Btwn<T>;
                fn expecting(
                    &self,
                    __f: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(__f, "struct Between")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(f0) = _serde::de::SeqAccess::next_element::<T>(&mut __seq)? else {
                        return Err(_serde::de::Error::invalid_length(
                            1usize,
                            &"struct Between with 2 els",
                        ));
                    };
                    let Some(f1) = _serde::de::SeqAccess::next_element::<T>(&mut __seq)? else {
                        return Err(_serde::de::Error::invalid_length(
                            2usize,
                            &"struct Between with 2 els",
                        ));
                    };
                    match Btwn::try_new(f0, f1) {
                        Ok(v) => Ok(v),
                        Err(er) => Err(serde::de::Error::custom(format!("{er:?}"))),
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
                                        <__A::Error as _serde::de::Error>::duplicate_field("start"),
                                    );
                                }
                                f0 = Some(_serde::de::MapAccess::next_value::<T>(&mut __map)?);
                            }
                            __Field::f1 => {
                                if Option::is_some(&f1) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("end"),
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
                        None => _serde::__private228::de::missing_field("start")?,
                    };
                    let f1_v = match f1 {
                        Some(v) => v,
                        None => _serde::__private228::de::missing_field("end")?,
                    };
                    match Btwn::try_new(f0_v, f1_v) {
                        Ok(v) => Ok(v),
                        Err(er) => Err(serde::de::Error::custom(format!("{er:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &[&str] = &["start", "end"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Between",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lt: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<
    T: pg_crud_cmn::DfltSomeOneEl
        + sqlx::Type<sqlx::Postgres>
        + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
> pg_crud_cmn::DfltSomeOneEl for Btwn<T>
{
    fn dflt_some_one_el() -> Self {
        Self {
            start: pg_crud_cmn::DfltSomeOneEl::dflt_some_one_el(),
            end: pg_crud_cmn::DfltSomeOneEl::dflt_some_one_el(),
        }
    }
}
impl<'lt, T: Send + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + 'lt>
    pg_crud_cmn::PgTypeWhFlt<'lt> for Btwn<T>
{
    fn qb(
        self,
        mut query: pg_crud_cmn::SqlxPostgresQuery<'lt>,
    ) -> Result<pg_crud_cmn::SqlxPostgresQuery<'lt>, pg_crud_cmn::SqlxPostgresQueryBindEr> {
        if let Err(er) = query.as_mut().try_bind(self.start) {
            return Err(
                match pg_crud_cmn::SqlxPostgresQueryBindEr::try_from(er.to_string()) {
                    Ok(v) => v,
                    Err(bind_er) => pg_crud_cmn::SqlxPostgresQueryBindEr::from(bind_er),
                },
            );
        }
        if let Err(er) = query.as_mut().try_bind(self.end) {
            return Err(
                match pg_crud_cmn::SqlxPostgresQueryBindEr::try_from(er.to_string()) {
                    Ok(v) => v,
                    Err(bind_er) => pg_crud_cmn::SqlxPostgresQueryBindEr::from(bind_er),
                },
            );
        }
        Ok(query)
    }
    fn qp(
        &self,
        incr: &mut dyn pg_crud_cmn::QpIncrMut,
        _: pg_crud_cmn::SqlColRef<'_>,
        _: pg_crud_cmn::AddOprtr,
    ) -> Result<pg_crud_cmn::QpFragment, pg_crud_cmn::QpEr> {
        let start_incr = match pg_crud_cmn::incr_checked_add_one_returning_incr(incr) {
            Ok(v) => v,
            Err(er) => {
                return Err(er);
            }
        };
        let end_incr = match pg_crud_cmn::incr_checked_add_one_returning_incr(incr) {
            Ok(v) => v,
            Err(er) => {
                return Err(er);
            }
        };
        let mut qp = String::with_capacity(32);
        if std::fmt::Write::write_fmt(
            &mut qp,
            format_args!("between ${start_incr} and ${end_incr}"),
        )
        .is_err()
        {
            return Err(pg_crud_cmn::QpEr::WriteIntoBuffer {
                loc: loc_macros::loc!(),
            });
        }
        Ok(pg_crud_cmn::QpFragment::try_from(qp)?)
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
)]
pub struct PgTypeNotEmptyUnqVec<T>(Vec<T>);
#[allow(clippy::arbitrary_source_item_ordering)]
impl<T> PgTypeNotEmptyUnqVec<T> {
    #[must_use]
    pub const fn to_vec(&self) -> &Vec<T> {
        &self.0
    }
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}
impl<T: PartialEq> TryFrom<Vec<T>> for PgTypeNotEmptyUnqVec<T> {
    type Error = pg_crud_cmn::NotEmptyUnqVecTryNewEr<T>;
    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        pg_crud_cmn::NotEmptyUnqVec::try_new(v)
            .map(pg_crud_cmn::NotEmptyUnqVec::into_vec)
            .map(Self)
    }
}
impl<T: Eq + std::hash::Hash> PgTypeNotEmptyUnqVec<T> {
    pub fn try_from_by_hash(v: Vec<T>) -> Result<Self, pg_crud_cmn::NotEmptyUnqVecTryNewEr<T>> {
        pg_crud_cmn::NotEmptyUnqVec::try_new_by_hash(v)
            .map(pg_crud_cmn::NotEmptyUnqVec::into_vec)
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
        for PgTypeNotEmptyUnqVec<T>
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
                marker: _serde::__private228::PhantomData<PgTypeNotEmptyUnqVec<T>>,
                lt: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T: std::fmt::Debug + PartialEq + _serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = PgTypeNotEmptyUnqVec<T>;
                fn expecting(
                    &self,
                    __f: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __f,
                        "tuple struct PgTypeNotEmptyUnqVec",
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let f0: Vec<T> = <Vec<T> as _serde::Deserialize>::deserialize(__e)?;
                    Ok(PgTypeNotEmptyUnqVec(f0))
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
                            &"tuple struct PgTypeNotEmptyUnqVec with 1 el",
                        ));
                    };
                    match PgTypeNotEmptyUnqVec::try_from(f0) {
                        Ok(v) => Ok(v),
                        Err(er) => Err(_serde::de::Error::custom(format!("{er:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "PgTypeNotEmptyUnqVec",
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lt: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<T: pg_crud_cmn::DfltSomeOneEl> pg_crud_cmn::DfltSomeOneEl for PgTypeNotEmptyUnqVec<T> {
    fn dflt_some_one_el() -> Self {
        Self(vec![pg_crud_cmn::DfltSomeOneEl::dflt_some_one_el()])
    }
}
impl<T> Default for PgTypeNotEmptyUnqVec<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}
impl<T> From<PgTypeNotEmptyUnqVec<T>> for Vec<T> {
    fn from(v: PgTypeNotEmptyUnqVec<T>) -> Self {
        v.0
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
)]
#[serde(try_from = "Vec<T>")]
pub struct BoundedVec<T, const LENGTH: usize>(Vec<T>);
#[location::errors_with_loc]
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
pub enum BoundedVecTryNewEr {
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
)]
pub struct BoundedVecLen(usize);
impl From<usize> for BoundedVecLen {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl BoundedVecLen {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}
impl std::fmt::Display for BoundedVecLen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl to_err_string::ToErrString for BoundedVecLen {
    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
        to_err_string::ToErrStringValue::try_from(self.to_string())
            .unwrap_or_else(to_err_string::ToErrStringValue::from)
    }
}
enum Vrt {
    MinusOne,
    Normal,
}
impl<
    'lt,
    T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + 'lt,
    const LENGTH: usize,
> BoundedVec<T, LENGTH>
{
    #[must_use]
    pub fn into_inn(self) -> Vec<T> {
        self.0
    }
    pub fn pg_type_qp(
        &self,
        incr: &mut dyn pg_crud_cmn::QpIncrMut,
        col: pg_crud_cmn::SqlColRef<'_>,
        add_oprtr: pg_crud_cmn::AddOprtr,
    ) -> Result<pg_crud_cmn::QpFragment, pg_crud_cmn::QpEr> {
        self.qp(incr, col, add_oprtr, &Vrt::Normal)
    }
    pub fn pg_type_qp_minus_one(
        &self,
        incr: &mut dyn pg_crud_cmn::QpIncrMut,
        col: pg_crud_cmn::SqlColRef<'_>,
        add_oprtr: pg_crud_cmn::AddOprtr,
    ) -> Result<pg_crud_cmn::QpFragment, pg_crud_cmn::QpEr> {
        self.qp(incr, col, add_oprtr, &Vrt::MinusOne)
    }
    pub fn qb(
        self,
        query: pg_crud_cmn::SqlxPostgresQuery<'lt>,
    ) -> Result<pg_crud_cmn::SqlxPostgresQuery<'lt>, pg_crud_cmn::SqlxPostgresQueryBindEr> {
        self.0.into_iter().try_fold(query, |mut acc_query, el| {
            acc_query.as_mut().try_bind(el).map_err(|er| {
                match pg_crud_cmn::SqlxPostgresQueryBindEr::try_from(er.to_string()) {
                    Ok(v) => v,
                    Err(bind_er) => pg_crud_cmn::SqlxPostgresQueryBindEr::from(bind_er),
                }
            })?;
            Ok(acc_query)
        })
    }
    fn qp(
        &self,
        incr: &mut dyn pg_crud_cmn::QpIncrMut,
        _: pg_crud_cmn::SqlColRef<'_>,
        _add_oprtr: pg_crud_cmn::AddOprtr,
        vrt: &Vrt,
    ) -> Result<pg_crud_cmn::QpFragment, pg_crud_cmn::QpEr> {
        let len = match &vrt {
            Vrt::MinusOne => self.0.len().saturating_sub(1),
            Vrt::Normal => self.0.len(),
        };
        let mut acc = String::with_capacity(len.saturating_mul(8));
        (0..len).try_for_each(|_| {
            let v = match pg_crud_cmn::incr_checked_add_one_returning_incr(incr) {
                Ok(v) => v,
                Err(er) => {
                    return Err(er);
                }
            };
            let write_res = std::fmt::Write::write_fmt(&mut acc, format_args!("[${v}]"));
            if write_res.is_err() {
                return Err(pg_crud_cmn::QpEr::WriteIntoBuffer {
                    loc: loc_macros::loc!(),
                });
            }
            Ok::<(), pg_crud_cmn::QpEr>(())
        })?;
        Ok(pg_crud_cmn::QpFragment::try_from(acc)?)
    }
    #[must_use]
    pub const fn to_inn(&self) -> &Vec<T> {
        &self.0
    }
}
impl<T, const LENGTH: usize> TryFrom<Vec<T>> for BoundedVec<T, LENGTH> {
    type Error = BoundedVecTryNewEr;
    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        let len = v.len();
        if len == LENGTH {
            Ok(Self(v))
        } else {
            Err(BoundedVecTryNewEr::LenIsNotCorrect {
                wrong_len: BoundedVecLen::from(len),
                expected: BoundedVecLen::from(LENGTH),
                loc: loc_macros::loc!(),
            })
        }
    }
}
impl<T: Clone + pg_crud_cmn::DfltSomeOneEl, const LENGTH: usize> pg_crud_cmn::DfltSomeOneEl
    for BoundedVec<T, LENGTH>
{
    fn dflt_some_one_el() -> Self {
        Self(vec![
            <T as pg_crud_cmn::DfltSomeOneEl>::dflt_some_one_el();
            LENGTH
        ])
    }
}
#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq, Eq)]
    struct NonClone(u8);
    #[test]
    fn pg_type_not_empty_unq_vec_try_from_ok() {
        let rslt = super::PgTypeNotEmptyUnqVec::<i32>::try_from(vec![1i32, 2i32, 3i32]);
        if let Err(er) = rslt {
            panic!("5a6afcfa {er:?}");
        }
    }
    #[test]
    fn pg_type_not_empty_unq_vec_try_from_empty() {
        let rslt = super::PgTypeNotEmptyUnqVec::<i32>::try_from(Vec::new());
        assert!(matches!(
            rslt,
            Err(pg_crud_cmn::NotEmptyUnqVecTryNewEr::IsEmpty { .. })
        ));
    }
    #[test]
    fn pg_type_not_empty_unq_vec_try_from_not_unq() {
        let rslt = super::PgTypeNotEmptyUnqVec::<i32>::try_from(vec![1i32, 2i32, 1i32]);
        assert!(matches!(
            rslt,
            Err(pg_crud_cmn::NotEmptyUnqVecTryNewEr::NotUnq { v: 1i32, .. })
        ));
    }
    #[test]
    fn pg_type_not_empty_unq_vec_try_from_by_hash_not_unq() {
        let rslt = super::PgTypeNotEmptyUnqVec::<i32>::try_from_by_hash(vec![1i32, 2i32, 1i32]);
        assert!(matches!(
            rslt,
            Err(pg_crud_cmn::NotEmptyUnqVecTryNewEr::NotUnq { v: 1i32, .. })
        ));
    }
    #[test]
    fn pg_type_not_empty_unq_vec_try_from_supports_non_clone_values() {
        let rslt = super::PgTypeNotEmptyUnqVec::<NonClone>::try_from(vec![
            NonClone(1),
            NonClone(2),
            NonClone(1),
        ]);
        assert!(matches!(
            rslt,
            Err(pg_crud_cmn::NotEmptyUnqVecTryNewEr::NotUnq { v: NonClone(1), .. })
        ));
    }
    #[test]
    fn encode_format_display_is_stable() {
        assert_eq!(super::EncodeFormat::Base64.to_string(), "base64");
        assert_eq!(super::EncodeFormat::Escape.to_string(), "escape");
        assert_eq!(super::EncodeFormat::Hex.to_string(), "hex");
    }
    #[test]
    fn rgx_rgx_eq_compares_pattern_content() {
        let left = super::RegexRgx::from(regex::Regex::new(r"\d+").expect("8342ad27"));
        let right = super::RegexRgx::from(regex::Regex::new(r"\d+").expect("4d0fa8e3"));
        let other = super::RegexRgx::from(regex::Regex::new("[a-z]+").expect("abcc9a72"));
        assert_eq!(left, right);
        assert_ne!(left, other);
    }
}
