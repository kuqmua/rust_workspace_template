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
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::DefaultInner,
    proc_macro_newtype::IntoVec,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub struct NotEmptyUniqueVec<T>(Vec<T>);
impl<T: utoipa::PartialSchema> utoipa::__dev::ComposeSchema for NotEmptyUniqueVec<T> {
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
            .max_items(Some(
                crate::not_empty_unique_vec_max_len::NOT_EMPTY_UNIQUE_VEC_MAX_LEN,
            ))
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
        duplicate_candidates: crate::duplicate_candidates::DuplicateCandidates<T>,
    ) -> Result<Self, crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError<T>>
    {
        crate::try_new_unique_vec::try_new_unique_vec(
            duplicate_candidates,
            crate::take_first_duplicate::take_first_duplicate,
        )
        .map(Self::from)
    }
}
impl<T: PartialEq> TryFrom<crate::duplicate_candidates::DuplicateCandidates<T>>
    for NotEmptyUniqueVec<T>
{
    type Error = crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError<T>;
    fn try_from(
        duplicate_candidates: crate::duplicate_candidates::DuplicateCandidates<T>,
    ) -> Result<Self, Self::Error> {
        Self::try_new(duplicate_candidates)
    }
}
impl<T: Eq + std::hash::Hash> NotEmptyUniqueVec<T> {
    pub fn try_new_by_hash(
        duplicate_candidates: crate::duplicate_candidates::DuplicateCandidates<T>,
    ) -> Result<Self, crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError<T>>
    {
        crate::try_new_unique_vec::try_new_unique_vec(
            duplicate_candidates,
            crate::take_first_duplicate_by_hash::take_first_duplicate_by_hash,
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
        fn deserialize<__D>(__d: __D) -> Result<Self, __D::Error>
        where
            __D: serde::Deserializer<'de>,
        {
            #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: serde::Deserialize<'de>,
            {
                marker: _serde::__private229::PhantomData<NotEmptyUniqueVec<T>>,
                lifetime_marker: _serde::__private229::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T: std::fmt::Debug + PartialEq + serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = NotEmptyUniqueVec<T>;
                fn expecting(
                    &self,
                    formatter: &mut std::fmt::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    std::fmt::Formatter::write_str(
                        formatter,
                        constants_str::PG_CRUD_NOT_EMPTY_UNIQUE_VEC_TUPLE_NAME,
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: serde::Deserializer<'de>,
                {
                    let f0: Vec<T> = <Vec<T> as serde::Deserialize>::deserialize(__e)?;
                    match NotEmptyUniqueVec::try_from(
                        crate::duplicate_candidates::DuplicateCandidates::from(f0),
                    ) {
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
                            &constants_str::PG_CRUD_NOT_EMPTY_UNIQUE_VEC_TUPLE_EXPECTING,
                        ));
                    };
                    match NotEmptyUniqueVec::try_from(
                        crate::duplicate_candidates::DuplicateCandidates::from(f0),
                    ) {
                        Ok(v) => Ok(v),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            serde::Deserializer::deserialize_newtype_struct(
                __d,
                constants_str::PG_CRUD_NOT_EMPTY_UNIQUE_VEC_SCHEMA_NAME,
                __Visitor {
                    marker: _serde::__private229::PhantomData::<Self>,
                    lifetime_marker: _serde::__private229::PhantomData,
                },
            )
        }
    }
};
impl<T: crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement> crate::default_some_one_element::DefaultSomeOneElement
    for NotEmptyUniqueVec<T>
{
    fn default_some_one_element() -> Self {
        Self::from(Vec::from(
            crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement::all_variants_default_some_one_element(
            ),
        ))
    }
}
impl<T: crate::all_enum_variants_array_default_some_one_element_max_page_size::AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize>
    crate::default_some_one_element_max_page_size::DefaultSomeOneElementMaxPageSize for NotEmptyUniqueVec<T>
{
    fn default_some_one_element_max_page_size() -> Self {
        Self::from(Vec::from(
            crate::all_enum_variants_array_default_some_one_element_max_page_size::AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize::all_variants_default_some_one_element_max_page_size(),
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
mod test_tests_not_empty_unique_vec {
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        PartialEq,
        Eq,
        proc_macro_newtype::FromInner,
    )]
    struct NonClone(u8);
    #[test]
    fn test_not_empty_unique_vec_try_new_supports_non_clone_values() {
        let error = crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new(
            vec![NonClone(1), NonClone(2), NonClone(1)].into(),
        )
        .expect_err(constants_str::ADF2B8C1);
        match error {
            crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::NotUnique { v, .. } => {
                assert_eq!(v, NonClone(1));
            }
            crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::IsEmpty { .. }
            | crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::TooLong { .. } => {
                std::panic::panic_any(constants_str::PANIC_9F5E2A34)
            }
        }
    }
    #[test]
    fn test_not_empty_unique_vec_rejects_oversized_and_deserialized_empty_values() {
        let oversized = (constants_usize::ZERO
            ..=crate::not_empty_unique_vec_max_len::NOT_EMPTY_UNIQUE_VEC_MAX_LEN)
            .collect::<Vec<_>>();
        assert!(matches!(
            crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new(oversized.into()),
            Err(
                crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::TooLong { .. }
            )
        ));
        let _error = serde_json::from_str::<crate::not_empty_unique_vec::NotEmptyUniqueVec<u8>>(
            constants_str::VALUE_4F53CDA1,
        )
        .expect_err(constants_str::VALUE_7C1A5E41);
    }
    #[test]
    fn test_not_empty_unique_vec_try_new_returns_is_empty_for_empty_vec() {
        let error =
            crate::not_empty_unique_vec::NotEmptyUniqueVec::<u8>::try_new(Vec::new().into())
                .expect_err(constants_str::VALUE_3B41DE7F);
        assert!(matches!(
            error,
            crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::IsEmpty { .. }
        ));
    }
    #[test]
    fn test_first_duplicate_idx_returns_none_for_unique_input() {
        let values = vec![1u8, 2u8, 3u8];
        assert!(crate::first_duplicate_index::first_duplicate_index(&values).is_none());
    }
    #[test]
    fn test_first_duplicate_idx_returns_none_for_empty_and_single_input() {
        assert!(crate::first_duplicate_index::first_duplicate_index::<u8>(&[]).is_none());
        assert!(crate::first_duplicate_index::first_duplicate_index(&[1u8]).is_none());
    }
    #[test]
    fn test_first_duplicate_idx_returns_fst_repeated_value_index() {
        let values = vec![7u8, 8u8, 8u8, 7u8];
        assert_eq!(
            crate::first_duplicate_index::first_duplicate_index(&values),
            Some(crate::duplicate_index::DuplicateIndex::from(2))
        );
    }
    #[test]
    fn test_first_duplicate_idx_by_hash_returns_fst_repeated_value_index() {
        let values = vec![7u8, 8u8, 8u8, 7u8];
        assert_eq!(
            crate::first_duplicate_index_by_hash::first_duplicate_index_by_hash(&values),
            Some(crate::duplicate_index::DuplicateIndex::from(2))
        );
    }
    #[test]
    fn test_first_duplicate_idx_by_hash_returns_none_for_empty_and_single_input() {
        assert!(
            crate::first_duplicate_index_by_hash::first_duplicate_index_by_hash::<u8>(&[])
                .is_none()
        );
        assert!(
            crate::first_duplicate_index_by_hash::first_duplicate_index_by_hash(&[1u8]).is_none()
        );
    }
    #[test]
    fn test_take_first_duplicate_returns_none_for_unique_input() {
        let mut values =
            crate::duplicate_candidates::DuplicateCandidates::from(vec![1u8, 2u8, 3u8]);
        let actual = crate::take_first_duplicate::take_first_duplicate(&mut values);
        assert!(actual.is_none());
        assert_eq!(Vec::from(values), [1u8, 2u8, 3u8]);
    }
    #[test]
    fn test_take_first_duplicate_returns_first_duplicate_value() {
        let mut values =
            crate::duplicate_candidates::DuplicateCandidates::from(vec![7u8, 8u8, 8u8, 7u8]);
        let actual = crate::take_first_duplicate::take_first_duplicate(&mut values);
        assert_eq!(actual, Some(8u8));
        assert_eq!(Vec::from(values).len(), 3usize);
    }
    #[test]
    fn test_take_first_duplicate_by_hash_returns_first_duplicate_value() {
        let mut values =
            crate::duplicate_candidates::DuplicateCandidates::from(vec![7u8, 8u8, 8u8, 7u8]);
        let actual = crate::take_first_duplicate_by_hash::take_first_duplicate_by_hash(&mut values);
        assert_eq!(actual, Some(8u8));
        assert_eq!(Vec::from(values).len(), 3usize);
    }
    #[test]
    fn test_not_empty_unique_vec_try_new_by_hash_returns_not_unique() {
        let error = crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new_by_hash(
            vec![1u8, 2u8, 1u8].into(),
        )
        .expect_err(constants_str::VALUE_59C80912);
        assert!(matches!(
            error,
            crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::NotUnique {
                v: 1u8,
                ..
            }
        ));
    }
    #[test]
    fn test_as_slice_matches_to_vec_view() {
        let values =
            crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new(vec![1u8, 2u8, 3u8].into())
                .expect(constants_str::DIAGNOSTIC_3F6E8A12);
        assert_eq!(values.as_slice(), &[1u8, 2u8, 3u8]);
        assert_eq!(values.as_slice(), &[1u8, 2u8, 3u8]);
    }
}
impl<'query_lt, T> crate::pg_type_where_filter::PgTypeWhereFilter<'query_lt> for NotEmptyUniqueVec<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> crate::pg_type_where_filter::PgTypeWhereFilter<'t_lt>
        + crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement,
{
    fn query_bind(
        self,
        sqlx_postgres_query: crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
    ) -> Result<crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>, crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError> {
        self.0
            .into_iter()
            .try_fold(sqlx_postgres_query, |accumulator_query, element| {
                element.query_bind(accumulator_query)
            })
    }
    fn query_part(
        &self,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
        sql_column_ref: crate::sql_column_ref::SqlColumnRef<'_>,
        add_operator: crate::add_operator::AddOperator,
    ) -> Result<crate::query_part_fragment::QueryPartFragment, crate::query_part_error::QueryPartError> {
        let mut accumulator = String::with_capacity(self.0.len().saturating_mul(32));
        self.0.iter().enumerate().try_for_each(|(i, element)| {
            let v = element.query_part(
                increment,
                sql_column_ref,
                if i == 0 {
                    add_operator
                } else {
                    crate::add_operator::AddOperator::from(true)
                },
            )?;
            accumulator.push_str(v.as_ref());
            Ok::<(), crate::query_part_error::QueryPartError>(())
        })?;
        Ok(crate::query_part_fragment::QueryPartFragment::try_from(accumulator)?)
    }
}
