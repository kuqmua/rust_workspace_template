use loc_lib::{Location, loc, loc::Loc};
use optml::Optml;
use pg_crud_cmn::{
    DfltSomeOneEl, NotEmptyUnqVec, NotEmptyUnqVecTryNewEr, PgTypeWhFlt, QpEr,
    incr_checked_add_one_returning_incr,
};
use regex::Regex;
use schemars::{_private::alloc::borrow, JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use sqlx::{Encode, Postgres, Type, postgres::PgArguments, query::Query};
use std::{
    fmt::{Display, Formatter, Result as StdFmtResult, Write as _},
    process::abort,
};
use thiserror::Error;
use utoipa::ToSchema;
gen_wh_flts::gen_wh_flts!({
    "pg_types_write_into_file": "False",
    "whole_write_into_file": "False"
});
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Optml)]
pub enum EncodeFormat {
    #[default]
    Base64,
    Escape,
    Hex,
}
impl Display for EncodeFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> StdFmtResult {
        match *self {
            Self::Base64 => write!(f, "base64"),
            Self::Escape => write!(f, "escape"),
            Self::Hex => write!(f, "hex"),
        }
    }
}
impl DfltSomeOneEl for EncodeFormat {
    fn dflt_some_one_el() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Optml)]
#[serde(try_from = "String", into = "String")]
pub struct RgxRgx(pub Regex);
impl TryFrom<String> for RgxRgx {
    type Error = regex::Error;
    fn try_from(v: String) -> Result<Self, Self::Error> {
        Regex::new(&v).map(RgxRgx)
    }
}
impl From<RgxRgx> for String {
    fn from(v: RgxRgx) -> Self {
        v.0.as_str().to_owned()
    }
}
// #[automatically_derived]
// impl ::core::marker::StructuralPartialEq for RegexRegex {}
// #[automatically_derived]
impl PartialEq for RgxRgx {
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
    impl JsonSchema for RgxRgx {
        fn schema_name() -> borrow::Cow<'static, str> {
            borrow::Cow::Borrowed("RegexRegex")
        }
        fn schema_id() -> borrow::Cow<'static, str> {
            borrow::Cow::Borrowed("tests::RegexRegex")
        }
        fn json_schema(generator: &mut SchemaGenerator) -> Schema {
            { generator.subschema_for::<String>() }
        }
        fn inline_schema() -> bool {
            false
        }
    }
};
impl Display for RgxRgx {
    fn fmt(&self, f: &mut Formatter<'_>) -> StdFmtResult {
        write!(f, "{}", self.0)
    }
}
impl DfltSomeOneEl for RgxRgx {
    fn dflt_some_one_el() -> Self {
        match Regex::new("[a-z]+") {
            Ok(v) => Self(v),
            Err(er) => {
                eprintln!("22a9eda5: {er}");
                abort();
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Optml)]
pub enum RgxCase {
    Insensitive,
    Sensitive,
}
impl DfltSomeOneEl for RgxCase {
    fn dflt_some_one_el() -> Self {
        Self::Sensitive
    }
}
impl RgxCase {
    #[must_use]
    pub const fn postgreql_syntax(&self) -> &'static str {
        match &self {
            Self::Insensitive => "~*",
            Self::Sensitive => "~",
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema, Optml)]
pub struct Btwn<T>
where
    T: Type<Postgres> + for<'__> Encode<'__, Postgres>,
{
    start: T,
    end: T,
}
#[derive(Debug, Clone, Serialize, Deserialize, Error, Location, Optml)]
pub enum BtwnTryNewEr<T> {
    StartMoreOrEqToEnd {
        #[eo_to_err_string_serde]
        start: T,
        #[eo_to_err_string_serde]
        end: T,
        loc: Loc,
    },
}
impl<T: Type<Postgres> + for<'__> Encode<'__, Postgres> + PartialOrd> Btwn<T> {
    pub fn try_new(start: T, end: T) -> Result<Self, BtwnTryNewEr<T>> {
        if start < end {
            Ok(Self { start, end })
        } else {
            Err(BtwnTryNewEr::StartMoreOrEqToEnd {
                start,
                end,
                loc: loc!(),
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
            + Type<Postgres>
            + for<'__> Encode<'__, Postgres>,
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
                T: _serde::Deserialize<'de> + Type<Postgres> + for<'__> Encode<'__, Postgres>,
            {
                marker: _serde::__private228::PhantomData<Btwn<T>>,
                lt: _serde::__private228::PhantomData<&'de ()>,
            }
            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
            where
                T: std::fmt::Debug
                    + _serde::Deserialize<'de>
                    + PartialOrd
                    + Type<Postgres>
                    + for<'__> Encode<'__, Postgres>,
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
impl<T: DfltSomeOneEl + Type<Postgres> + for<'__> Encode<'__, Postgres>> DfltSomeOneEl for Btwn<T> {
    fn dflt_some_one_el() -> Self {
        Self {
            start: DfltSomeOneEl::dflt_some_one_el(),
            end: DfltSomeOneEl::dflt_some_one_el(),
        }
    }
}
impl<'lt, T: Send + Type<Postgres> + for<'__> Encode<'__, Postgres> + 'lt> PgTypeWhFlt<'lt>
    for Btwn<T>
{
    fn qb(
        self,
        mut query: Query<'lt, Postgres, PgArguments>,
    ) -> Result<Query<'lt, Postgres, PgArguments>, String> {
        if let Err(er) = query.try_bind(self.start) {
            return Err(er.to_string());
        }
        if let Err(er) = query.try_bind(self.end) {
            return Err(er.to_string());
        }
        Ok(query)
    }
    fn qp(&self, incr: &mut u64, _: &dyn Display, _: bool) -> Result<String, QpEr> {
        let start_incr = match incr_checked_add_one_returning_incr(incr) {
            Ok(v) => v,
            Err(er) => {
                return Err(er);
            }
        };
        let end_incr = match incr_checked_add_one_returning_incr(incr) {
            Ok(v) => v,
            Err(er) => {
                return Err(er);
            }
        };
        Ok(format!("between ${start_incr} and ${end_incr}"))
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema, JsonSchema, Optml)]
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
    type Error = NotEmptyUnqVecTryNewEr<T>;
    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        NotEmptyUnqVec::try_new(v)
            .map(NotEmptyUnqVec::into_vec)
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
impl<T: DfltSomeOneEl> DfltSomeOneEl for PgTypeNotEmptyUnqVec<T> {
    fn dflt_some_one_el() -> Self {
        Self(vec![DfltSomeOneEl::dflt_some_one_el()])
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
    Debug, Default, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, JsonSchema, Optml,
)]
#[serde(try_from = "Vec<T>")]
pub struct BoundedVec<T, const LENGTH: usize>(Vec<T>);
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error, Location, JsonSchema, Optml,
)]
pub enum BoundedVecTryNewEr {
    LenIsNotCorrect {
        #[eo_to_err_string_serde]
        wrong_len: usize,
        #[eo_to_err_string_serde]
        expected: usize,
        loc: Loc,
    },
}
enum Vrt {
    MinusOne,
    Normal,
}
impl<'lt, T: Type<Postgres> + for<'__> Encode<'__, Postgres> + 'lt, const LENGTH: usize>
    BoundedVec<T, LENGTH>
{
    #[must_use]
    pub fn into_inn(self) -> Vec<T> {
        self.0
    }
    pub fn pg_type_qp(
        &self,
        incr: &mut u64,
        col: &dyn Display,
        add_oprtr: bool,
    ) -> Result<String, QpEr> {
        self.qp(incr, col, add_oprtr, &Vrt::Normal)
    }
    pub fn pg_type_qp_minus_one(
        &self,
        incr: &mut u64,
        col: &dyn Display,
        add_oprtr: bool,
    ) -> Result<String, QpEr> {
        self.qp(incr, col, add_oprtr, &Vrt::MinusOne)
    }
    pub fn qb(
        self,
        mut query: Query<'lt, Postgres, PgArguments>,
    ) -> Result<Query<'lt, Postgres, PgArguments>, String> {
        for el in self.0 {
            if let Err(er) = query.try_bind(el) {
                return Err(er.to_string());
            }
        }
        Ok(query)
    }
    fn qp(
        &self,
        incr: &mut u64,
        _: &dyn Display,
        _add_oprtr: bool,
        vrt: &Vrt,
    ) -> Result<String, QpEr> {
        let mut acc = String::new();
        let len = match &vrt {
            Vrt::MinusOne => self.0.len().saturating_sub(1),
            Vrt::Normal => self.0.len(),
        };
        for _ in 0..len {
            let v = match incr_checked_add_one_returning_incr(incr) {
                Ok(v) => v,
                Err(er) => {
                    return Err(er);
                }
            };
            let write_res = write!(acc, "[${v}]");
            if write_res.is_err() {
                return Err(QpEr::WriteIntoBuffer { loc: loc!() });
            }
        }
        Ok(acc)
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
                wrong_len: len,
                expected: LENGTH,
                loc: loc!(),
            })
        }
    }
}
impl<T: Clone + DfltSomeOneEl, const LENGTH: usize> DfltSomeOneEl for BoundedVec<T, LENGTH> {
    fn dflt_some_one_el() -> Self {
        Self(vec![<T as DfltSomeOneEl>::dflt_some_one_el(); LENGTH])
    }
}
#[cfg(test)]
mod tests {
    use super::{EncodeFormat, PgTypeNotEmptyUnqVec, RgxRgx};
    use pg_crud_cmn::NotEmptyUnqVecTryNewEr;
    use regex::Regex;
    #[derive(Debug, PartialEq, Eq)]
    struct NonClone(u8);
    #[test]
    fn pg_type_not_empty_unq_vec_try_from_ok() {
        let rslt = PgTypeNotEmptyUnqVec::<i32>::try_from(vec![1i32, 2i32, 3i32]);
        if let Err(er) = rslt {
            panic!("5a6afcfa {er:?}");
        }
    }
    #[test]
    fn pg_type_not_empty_unq_vec_try_from_empty() {
        let rslt = PgTypeNotEmptyUnqVec::<i32>::try_from(Vec::new());
        assert!(matches!(rslt, Err(NotEmptyUnqVecTryNewEr::IsEmpty { .. })));
    }
    #[test]
    fn pg_type_not_empty_unq_vec_try_from_not_unq() {
        let rslt = PgTypeNotEmptyUnqVec::<i32>::try_from(vec![1i32, 2i32, 1i32]);
        assert!(matches!(
            rslt,
            Err(NotEmptyUnqVecTryNewEr::NotUnq { v: 1i32, .. })
        ));
    }
    #[test]
    fn pg_type_not_empty_unq_vec_try_from_supports_non_clone_values() {
        let rslt =
            PgTypeNotEmptyUnqVec::<NonClone>::try_from(vec![NonClone(1), NonClone(2), NonClone(1)]);
        assert!(matches!(
            rslt,
            Err(NotEmptyUnqVecTryNewEr::NotUnq { v: NonClone(1), .. })
        ));
    }
    #[test]
    fn encode_format_display_is_stable() {
        assert_eq!(EncodeFormat::Base64.to_string(), "base64");
        assert_eq!(EncodeFormat::Escape.to_string(), "escape");
        assert_eq!(EncodeFormat::Hex.to_string(), "hex");
    }
    #[test]
    fn rgx_rgx_eq_compares_pattern_content() {
        let left = RgxRgx(Regex::new(r"\d+").expect("8342ad27"));
        let right = RgxRgx(Regex::new(r"\d+").expect("4d0fa8e3"));
        let other = RgxRgx(Regex::new("[a-z]+").expect("abcc9a72"));
        assert_eq!(left, right);
        assert_ne!(left, other);
    }
}
