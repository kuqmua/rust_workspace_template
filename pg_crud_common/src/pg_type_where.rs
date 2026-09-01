// The owner module retains lint-sensitive semantics from the original implementation.
#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "generated getter field order preserves the established serialized contract"
)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct PgTypeWhere<T> {
    values: crate::not_empty_unique_vec::NotEmptyUniqueVec<T>,
    operator: crate::operator::Operator,
}
impl<T: utoipa::PartialSchema> utoipa::__dev::ComposeSchema for PgTypeWhere<T> {
    fn compose(
        _new_generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                constants_str::PG_CRUD_VALUES_FIELD,
                <crate::not_empty_unique_vec::NotEmptyUniqueVec<T> as utoipa::PartialSchema>::schema(),
            )
            .property(
                constants_str::PG_CRUD_OPERATOR_FIELD,
                <crate::operator::Operator as utoipa::PartialSchema>::schema(),
            )
            .required(constants_str::PG_CRUD_VALUES_FIELD)
            .required(constants_str::PG_CRUD_OPERATOR_FIELD)
            .build()
            .into()
    }
}
impl<T: utoipa::ToSchema> utoipa::ToSchema for PgTypeWhere<T> {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::PG_CRUD_PG_TYPE_WHERE_SCHEMA_NAME)
    }
}
impl<T: PartialEq + Clone> PgTypeWhere<T> {
    #[must_use]
    pub const fn new(
        operator: crate::operator::Operator,
        values: crate::not_empty_unique_vec::NotEmptyUniqueVec<T>,
    ) -> Self {
        Self { values, operator }
    }

    pub fn try_new(
        operator: crate::operator::Operator,
        values: crate::duplicate_candidates::DuplicateCandidates<T>,
    ) -> Result<Self, crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError<T>>
    {
        match crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new(values) {
            Ok(validated_values) => Ok(Self {
                values: validated_values,
                operator,
            }),
            Err(error) => Err(error),
        }
    }
}
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(unused_qualifications)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::absolute_paths)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[expect(clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + PartialEq + Clone + serde::Deserialize<'de>>
        serde::Deserialize<'de> for PgTypeWhere<T>
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: serde::Deserializer<'de>,
        {
            #[derive(optimal_memory_layout::OptimalMemoryLayout)]
            // The owner module retains lint-sensitive semantics from the original implementation.
            #[expect(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                f0,
                f1,
                __ignore,
            }
            #[derive(optimal_memory_layout::OptimalMemoryLayout)]
            #[doc(hidden)]
            struct __FieldVisitor;
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __f: &mut std::fmt::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(
                        __f,
                        constants_str::PG_CRUD_FIELD_IDENTIFIER,
                    )
                }
                fn visit_u64<__E>(self, v: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match v {
                        constants_u64::ZERO => Ok(__Field::f0),
                        1u64 => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, v: &str) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match v {
                        constants_str::PG_CRUD_OPERATOR_FIELD => Ok(__Field::f0),
                        constants_str::PG_CRUD_VALUES_FIELD => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, v: &[u8]) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match v {
                        b"operator" => Ok(__Field::f0),
                        b"values" => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                where
                    __D: serde::Deserializer<'de>,
                {
                    serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[derive(optimal_memory_layout::OptimalMemoryLayout)]
            #[doc(hidden)]
            struct __Visitor<'de, PgTypeWhere> {
                marker: _serde::__private229::PhantomData<PgTypeWhere>,
                lifetime_marker: _serde::__private229::PhantomData<&'de ()>,
            }
            impl<'de, T: std::fmt::Debug + PartialEq + Clone + serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = PgTypeWhere<T>;
                fn expecting(
                    &self,
                    __f: &mut std::fmt::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    std::fmt::Formatter::write_str(
                        __f,
                        constants_str::PG_CRUD_PG_TYPE_WHERE_STRUCT_NAME,
                    )
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(f0) = _serde::de::SeqAccess::next_element::<crate::operator::Operator>(
                        &mut __seq,
                    )?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            constants_usize::ZERO,
                            &constants_str::PG_CRUD_PG_TYPE_WHERE_EXPECTING,
                        ));
                    };
                    let Some(f1) = _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            constants_usize::ONE,
                            &constants_str::PG_CRUD_PG_TYPE_WHERE_EXPECTING,
                        ));
                    };
                    match PgTypeWhere::try_new(f0, f1.into()) {
                        Ok(v) => Ok(v),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut f0: Option<crate::operator::Operator> = None;
                    let mut f1: Option<Vec<T>> = None;
                    while let Some(__k) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __k {
                            __Field::f0 => {
                                if Option::is_some(&f0) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            constants_str::PG_CRUD_OPERATOR_FIELD,
                                        ),
                                    );
                                }
                                f0 = Some(_serde::de::MapAccess::next_value::<
                                    crate::operator::Operator,
                                >(&mut __map)?);
                            }
                            __Field::f1 => {
                                if Option::is_some(&f1) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            constants_str::PG_CRUD_VALUES_FIELD,
                                        ),
                                    );
                                }
                                f1 = Some(_serde::de::MapAccess::next_value::<Vec<T>>(&mut __map)?);
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
                        None => _serde::__private229::de::missing_field(
                            constants_str::PG_CRUD_OPERATOR_FIELD,
                        )?,
                    };
                    let f1_v = match f1 {
                        Some(v) => v,
                        None => _serde::__private229::de::missing_field(
                            constants_str::PG_CRUD_VALUES_FIELD,
                        )?,
                    };
                    match PgTypeWhere::try_new(f0_v, f1_v.into()) {
                        Ok(v) => Ok(v),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            serde::Deserializer::deserialize_struct(
                __deserializer,
                constants_str::PG_CRUD_PG_TYPE_WHERE_SCHEMA_NAME,
                constants_str::PG_CRUD_SERDE_PG_TYPE_WHERE_FIELDS,
                __Visitor {
                    marker: _serde::__private229::PhantomData::<T>,
                    lifetime_marker: _serde::__private229::PhantomData,
                },
            )
        }
    }
};
impl<'query_lt, T: crate::pg_type_where_filter::PgTypeWhereFilter<'query_lt>>
    crate::pg_type_where_filter::PgTypeWhereFilter<'query_lt> for PgTypeWhere<T>
{
    fn query_bind(
        self,
        query: crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
    ) -> Result<
        crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
        crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    > {
        self.values
            .into_vec()
            .into_iter()
            .try_fold(query, |accumulator_query, element| {
                crate::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                    element,
                    accumulator_query,
                )
            })
    }
    fn query_part(
        &self,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
        column: crate::sql_column_ref::SqlColumnRef<'_>,
        add_operator: crate::add_operator::AddOperator,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    > {
        let operator_query_part = self.operator.to_query_part(add_operator);
        let mut query_part = String::with_capacity(
            operator_query_part
                .as_ref()
                .len()
                .saturating_add(self.values.as_slice().len().saturating_mul(32))
                .saturating_add(2),
        );
        query_part.push_str(operator_query_part.as_ref());
        query_part.push('(');
        let mut element_add_operator = crate::add_operator::AddOperator::from(false);
        let mut is_first = true;
        self.values.as_slice().iter().try_for_each(|element| {
            let v = crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
                element,
                increment,
                column,
                element_add_operator,
            )?;
            if is_first {
                is_first = false;
            } else {
                query_part.push(' ');
            }
            query_part.push_str(v.as_ref());
            element_add_operator = crate::add_operator::AddOperator::from(true);
            Ok::<(), crate::query_part_error::QueryPartError>(())
        })?;
        query_part.push(')');
        Ok(
            crate::query_part_fragment::QueryPartFragment::try_from(query_part)
                .unwrap_or_else(crate::query_part_fragment::QueryPartFragment::from),
        )
    }
}
impl<T: std::fmt::Debug + PartialEq + Clone + crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement>
    crate::default_some_one_element::DefaultSomeOneElement for PgTypeWhere<T>
{
    fn default_some_one_element() -> Self {
        Self {
            operator: crate::default_some_one_element::DefaultSomeOneElement::default_some_one_element(),
            values: crate::default_some_one_element::DefaultSomeOneElement::default_some_one_element(),
        }
    }
}
