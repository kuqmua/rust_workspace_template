#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    schemars::JsonSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::AsSlice,
    proc_macro_newtype::IntoInnerFrom,
)]
pub struct PgTypeNotEmptyUniqueVec<T>(Vec<T>);
impl<T> From<[T; 1]> for PgTypeNotEmptyUniqueVec<T> {
    fn from(value: [T; 1]) -> Self {
        Self(Vec::from(value))
    }
}
impl<T: utoipa::PartialSchema> utoipa::__dev::ComposeSchema for PgTypeNotEmptyUniqueVec<T> {
    #[allow(
        unused_variables,
        reason = "the schema trait implementation preserves the type-based parameter name"
    )]
    fn compose(
        vec: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ArrayBuilder::new()
            .items(<T as utoipa::PartialSchema>::schema())
            .min_items(Some(1))
            .build()
            .into()
    }
}
impl<T: utoipa::ToSchema> utoipa::ToSchema for PgTypeNotEmptyUniqueVec<T> {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::PG_CRUD_PG_TYPE_NOT_EMPTY_UNIQUE_VEC_SCHEMA_NAME)
    }
}
impl<T: PartialEq> TryFrom<Vec<T>> for PgTypeNotEmptyUniqueVec<T> {
    type Error =
        pg_crud_common::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError<T>;
    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        pg_crud_common::not_empty_unique_vec::NotEmptyUniqueVec::try_new(vec.into())
            .map(pg_crud_common::not_empty_unique_vec::NotEmptyUniqueVec::into_vec)
            .map(Self)
    }
}
impl<T: Eq + std::hash::Hash> PgTypeNotEmptyUniqueVec<T> {
    pub fn try_from_by_hash(
        duplicate_candidates: pg_crud_common::duplicate_candidates::DuplicateCandidates<T>,
    ) -> Result<
        Self,
        pg_crud_common::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError<T>,
    > {
        pg_crud_common::not_empty_unique_vec::NotEmptyUniqueVec::try_new_by_hash(
            duplicate_candidates,
        )
        .map(pg_crud_common::not_empty_unique_vec::NotEmptyUniqueVec::into_vec)
        .map(Self)
    }
}

#[allow(unused_qualifications, reason = "lint suppression is required here")]
#[allow(clippy::absolute_paths, reason = "lint suppression is required here")]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "lint suppression is required here"
)]
const _: () = {
    #[expect(
        clippy::useless_attribute,
        reason = "lint suppression is required here"
    )]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + PartialEq + _serde::Deserialize<'de>> _serde::Deserialize<'de>
        for PgTypeNotEmptyUniqueVec<T>
    {
        fn deserialize<__D>(__d: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private229::PhantomData<PgTypeNotEmptyUniqueVec<T>>,
                lifetime_marker: _serde::__private229::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T: std::fmt::Debug + PartialEq + _serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = PgTypeNotEmptyUniqueVec<T>;
                fn expecting(
                    &self,
                    formatter: &mut _serde::__private229::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(
                        formatter,
                        constants_str::PG_CRUD_PG_TYPE_NOT_EMPTY_UNIQUE_VEC_TUPLE_NAME,
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
                fn visit_seq<__A>(self, mut __a: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(f0) = _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __a)? else {
                        return Err(_serde::de::Error::invalid_length(
                            constants_usize::ZERO,
                            &constants_str::PG_CRUD_PG_TYPE_NOT_EMPTY_UNIQUE_VEC_TUPLE_EXPECTING,
                        ));
                    };
                    match PgTypeNotEmptyUniqueVec::try_from(f0) {
                        Ok(v) => Ok(v),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __d,
                constants_str::PG_CRUD_PG_TYPE_NOT_EMPTY_UNIQUE_VEC_SCHEMA_NAME,
                __Visitor {
                    marker: _serde::__private229::PhantomData::<Self>,
                    lifetime_marker: _serde::__private229::PhantomData,
                },
            )
        }
    }
};
impl<T: pg_crud_common::default_some_one_element::DefaultSomeOneElement>
    pg_crud_common::default_some_one_element::DefaultSomeOneElement for PgTypeNotEmptyUniqueVec<T>
{
    fn default_some_one_element() -> Self {
        Self::from([
            pg_crud_common::default_some_one_element::DefaultSomeOneElement::default_some_one_element(),
        ])
    }
}
