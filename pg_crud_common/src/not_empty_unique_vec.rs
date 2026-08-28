#![allow(
    clippy::items_after_test_module,
    reason = "generated filter implementations remain after focused collection tests"
)]

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DefaultInner,
    newtype::IntoVec,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct NotEmptyUniqueVec<T>(Vec<T>);
impl<T: utoipa::PartialSchema> utoipa::__dev::ComposeSchema for NotEmptyUniqueVec<T> {
    fn compose(
        _new_generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ArrayBuilder::new()
            .items(<T as utoipa::PartialSchema>::schema())
            .min_items(Some(1))
            .max_items(Some(super::NOT_EMPTY_UNIQUE_VEC_MAX_LEN))
            .build()
            .into()
    }
}
impl<T: utoipa::ToSchema> utoipa::ToSchema for NotEmptyUniqueVec<T> {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::PG_CRUD_NOT_EMPTY_UNIQUE_VEC_SCHEMA_NAME)
    }
}
impl<T> NotEmptyUniqueVec<T> {
    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}
impl<T: PartialEq> NotEmptyUniqueVec<T> {
    pub fn try_new(
        values: super::DuplicateCandidates<T>,
    ) -> Result<Self, crate::domain_types::NotEmptyUniqueVecTryNewError<T>> {
        crate::try_new_unique_vec::try_new_unique_vec(values, crate::domain_types::take_fst_dup)
            .map(Self::from)
    }
}
impl<T: PartialEq> TryFrom<super::DuplicateCandidates<T>> for NotEmptyUniqueVec<T> {
    type Error = crate::domain_types::NotEmptyUniqueVecTryNewError<T>;
    fn try_from(value: super::DuplicateCandidates<T>) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}
impl<T: Eq + std::hash::Hash> NotEmptyUniqueVec<T> {
    pub fn try_new_by_hash(
        values: super::DuplicateCandidates<T>,
    ) -> Result<Self, crate::domain_types::NotEmptyUniqueVecTryNewError<T>> {
        crate::try_new_unique_vec::try_new_unique_vec(
            values,
            crate::domain_types::take_fst_dup_by_hash,
        )
        .map(Self::from)
    }
}
#[allow(
    unused_qualifications,
    reason = "generated deserialization keeps fully qualified compatibility paths"
)]
#[allow(
    clippy::absolute_paths,
    reason = "generated deserialization uses absolute serde compatibility paths"
)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "generated deserialization remains adjacent to its collection type"
)]
const _: () = {
    #[expect(
        clippy::useless_attribute,
        reason = "generated serde compatibility import may be redundant on some toolchains"
    )]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + PartialEq + serde::Deserialize<'de>> serde::Deserialize<'de>
        for NotEmptyUniqueVec<T>
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: serde::Deserializer<'de>,
        {
            #[derive(optimal_memory_layout::OptimalMemoryLayout)]
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: serde::Deserialize<'de>,
            {
                marker: _serde::__private229::PhantomData<NotEmptyUniqueVec<T>>,
                lt: _serde::__private229::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T: std::fmt::Debug + PartialEq + serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = NotEmptyUniqueVec<T>;
                fn expecting(
                    &self,
                    __f: &mut std::fmt::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    std::fmt::Formatter::write_str(
                        __f,
                        constants_str::PG_CRUD_NOT_EMPTY_UNIQUE_VEC_TUPLE_NAME,
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: serde::Deserializer<'de>,
                {
                    let f0: Vec<T> = <Vec<T> as serde::Deserialize>::deserialize(__e)?;
                    match NotEmptyUniqueVec::try_from(super::DuplicateCandidates::from(f0)) {
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
                            constants_usize::ZERO,
                            &constants_str::PG_CRUD_NOT_EMPTY_UNIQUE_VEC_TUPLE_EXPECTING,
                        ));
                    };
                    match NotEmptyUniqueVec::try_from(super::DuplicateCandidates::from(f0)) {
                        Ok(v) => Ok(v),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                constants_str::PG_CRUD_NOT_EMPTY_UNIQUE_VEC_SCHEMA_NAME,
                __Visitor {
                    marker: _serde::__private229::PhantomData::<Self>,
                    lt: _serde::__private229::PhantomData,
                },
            )
        }
    }
};
impl<T: super::AllEnumVariantsArrayDefaultSomeOneElement> super::DefaultSomeOneElement
    for NotEmptyUniqueVec<T>
{
    fn default_some_one_element() -> Self {
        Self::from(Vec::from(
            super::AllEnumVariantsArrayDefaultSomeOneElement::all_variants_default_some_one_element(
            ),
        ))
    }
}
impl<T: super::AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize>
    super::DefaultSomeOneElementMaxPageSize for NotEmptyUniqueVec<T>
{
    fn default_some_one_element_max_page_size() -> Self {
        Self::from(Vec::from(
            super::AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize::all_variants_default_some_one_element_max_page_size(),
        ))
    }
}
impl<T1> NotEmptyUniqueVec<T1> {
    pub fn from_t1_impl_from_t2<T2>(v: Self) -> NotEmptyUniqueVec<T2>
    where
        T2: From<T1>,
    {
        NotEmptyUniqueVec::from(v.0.into_iter().map(T2::from).collect::<Vec<T2>>())
    }
}
#[cfg(test)]
mod tests_not_empty_unique_vec {
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout, Debug, PartialEq, Eq, newtype::FromInner,
    )]
    struct NonClone(u8);
    #[test]
    fn not_empty_unique_vec_try_new_supports_non_clone_values() {
        let error =
            super::NotEmptyUniqueVec::try_new(vec![NonClone(1), NonClone(2), NonClone(1)].into())
                .expect_err(constants_str::ADF2B8C1);
        match error {
            crate::domain_types::NotEmptyUniqueVecTryNewError::NotUnique { v, .. } => {
                assert_eq!(v, NonClone(1));
            }
            crate::domain_types::NotEmptyUniqueVecTryNewError::IsEmpty { .. }
            | crate::domain_types::NotEmptyUniqueVecTryNewError::TooLong { .. } => {
                panic!("9f5e2a34")
            }
        }
    }
    #[test]
    fn not_empty_unique_vec_rejects_oversized_and_deserialized_empty_values() {
        let oversized = (constants_usize::ZERO..=super::super::NOT_EMPTY_UNIQUE_VEC_MAX_LEN)
            .collect::<Vec<_>>();
        assert!(matches!(
            super::NotEmptyUniqueVec::try_new(oversized.into()),
            Err(crate::domain_types::NotEmptyUniqueVecTryNewError::TooLong { .. })
        ));
        let _error =
            serde_json::from_str::<super::NotEmptyUniqueVec<u8>>(constants_str::VALUE_4F53CDA1)
                .expect_err(constants_str::VALUE_7C1A5E41);
    }
    #[test]
    fn not_empty_unique_vec_try_new_returns_is_empty_for_empty_vec() {
        let error = super::NotEmptyUniqueVec::<u8>::try_new(Vec::new().into())
            .expect_err(constants_str::VALUE_3B41DE7F);
        assert!(matches!(
            error,
            crate::domain_types::NotEmptyUniqueVecTryNewError::IsEmpty { .. }
        ));
    }
    #[test]
    fn fst_dup_idx_returns_none_for_unique_input() {
        let values = vec![1u8, 2u8, 3u8];
        assert!(crate::domain_types::first_duplicate_index(&values).is_none());
    }
    #[test]
    fn fst_dup_idx_returns_none_for_empty_and_single_input() {
        assert!(crate::first_duplicate_index::<u8>(&[]).is_none());
        assert!(crate::domain_types::first_duplicate_index(&[1u8]).is_none());
    }
    #[test]
    fn fst_dup_idx_returns_fst_repeated_value_idx() {
        let values = vec![7u8, 8u8, 8u8, 7u8];
        assert_eq!(
            crate::domain_types::first_duplicate_index(&values),
            Some(crate::domain_types::DuplicateIdx::from(2))
        );
    }
    #[test]
    fn fst_dup_idx_by_hash_returns_fst_repeated_value_idx() {
        let values = vec![7u8, 8u8, 8u8, 7u8];
        assert_eq!(
            crate::domain_types::first_duplicate_index_by_hash(&values),
            Some(crate::domain_types::DuplicateIdx::from(2))
        );
    }
    #[test]
    fn fst_dup_idx_by_hash_returns_none_for_empty_and_single_input() {
        assert!(crate::first_duplicate_index_by_hash::<u8>(&[]).is_none());
        assert!(crate::domain_types::first_duplicate_index_by_hash(&[1u8]).is_none());
    }
    #[test]
    fn take_fst_dup_returns_none_for_unique_input() {
        let mut values = super::super::DuplicateCandidates::from(vec![1u8, 2u8, 3u8]);
        let actual = crate::domain_types::take_fst_dup(&mut values);
        assert!(actual.is_none());
        assert_eq!(Vec::from(values), vec![1u8, 2u8, 3u8]);
    }
    #[test]
    fn take_fst_dup_returns_first_duplicate_value() {
        let mut values = super::super::DuplicateCandidates::from(vec![7u8, 8u8, 8u8, 7u8]);
        let actual = crate::domain_types::take_fst_dup(&mut values);
        assert_eq!(actual, Some(8u8));
        assert_eq!(Vec::from(values).len(), 3usize);
    }
    #[test]
    fn take_fst_dup_by_hash_returns_first_duplicate_value() {
        let mut values = super::super::DuplicateCandidates::from(vec![7u8, 8u8, 8u8, 7u8]);
        let actual = crate::domain_types::take_fst_dup_by_hash(&mut values);
        assert_eq!(actual, Some(8u8));
        assert_eq!(Vec::from(values).len(), 3usize);
    }
    #[test]
    fn not_empty_unique_vec_try_new_by_hash_returns_not_unique() {
        let error = super::NotEmptyUniqueVec::try_new_by_hash(vec![1u8, 2u8, 1u8].into())
            .expect_err(constants_str::VALUE_59C80912);
        assert!(matches!(
            error,
            crate::domain_types::NotEmptyUniqueVecTryNewError::NotUnique { v: 1u8, .. }
        ));
    }
    #[test]
    fn as_slice_matches_to_vec_view() {
        let values = super::NotEmptyUniqueVec::try_new(vec![1u8, 2u8, 3u8].into())
            .expect("3f6e8a12 as_slice_matches_to_vec_view invariant must hold");
        assert_eq!(values.as_slice(), &[1u8, 2u8, 3u8]);
        assert_eq!(values.as_slice(), &[1u8, 2u8, 3u8]);
    }
}
impl<'query_lt, T> super::PgTypeWhereFilter<'query_lt> for NotEmptyUniqueVec<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> super::PgTypeWhereFilter<'t_lt>
        + super::AllEnumVariantsArrayDefaultSomeOneElement,
{
    fn query_bind(
        self,
        query: super::SqlxPostgresQuery<'query_lt>,
    ) -> Result<super::SqlxPostgresQuery<'query_lt>, super::SqlxPostgresQueryBindError> {
        self.0
            .into_iter()
            .try_fold(query, |accumulator_query, element| {
                element.query_bind(accumulator_query)
            })
    }
    fn query_part(
        &self,
        increment: &mut dyn super::QueryPartIncrementMut,
        column: super::SqlColumnRef<'_>,
        add_operator: super::AddOperator,
    ) -> Result<super::QueryPartFragment, super::QueryPartError> {
        let mut accumulator = String::with_capacity(self.0.len().saturating_mul(32));
        self.0.iter().enumerate().try_for_each(|(i, element)| {
            let v = element.query_part(
                increment,
                column,
                if i == 0 {
                    add_operator
                } else {
                    super::AddOperator::from(true)
                },
            )?;
            accumulator.push_str(v.as_ref());
            Ok::<(), super::QueryPartError>(())
        })?;
        Ok(super::QueryPartFragment::try_from(accumulator)?)
    }
}
