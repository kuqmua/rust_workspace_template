#[derive(proc_macro_getters::Getters)]
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
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct PgTypeWhere<T> {
    values: crate::not_empty_unique_vec::NotEmptyUniqueVec<T>,
    operator: crate::operator::Operator,
}
impl<T: utoipa::PartialSchema> utoipa::__dev::ComposeSchema for PgTypeWhere<T> {
    #[allow(
        unused_variables,
        reason = "the schema trait implementation preserves the type-based parameter name"
    )]
    fn compose(
        vec: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
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
        not_empty_unique_vec: crate::not_empty_unique_vec::NotEmptyUniqueVec<T>,
    ) -> Self {
        Self {
            values: not_empty_unique_vec,
            operator,
        }
    }

    pub fn try_new(
        operator: crate::operator::Operator,
        duplicate_candidates: crate::duplicate_candidates::DuplicateCandidates<T>,
    ) -> Result<Self, crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError<T>>
    {
        match crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new(duplicate_candidates) {
            Ok(validated_values) => Ok(Self {
                values: validated_values,
                operator,
            }),
            Err(error) => Err(error),
        }
    }
}

#[allow(
    unused_qualifications,
    reason = "pg type where keeps explicit generated paths stable across expansion contexts"
)]
#[allow(
    clippy::absolute_paths,
    reason = "pg type where uses explicit paths to comply with the workspace import policy"
)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "pg type where keeps declaration order aligned with generated layout or processing flow"
)]
const _: () = {
    #[expect(
        clippy::useless_attribute,
        reason = "pg type where keeps declaration order aligned with generated layout or processing flow"
    )]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + PartialEq + Clone + serde::Deserialize<'de>>
        serde::Deserialize<'de> for PgTypeWhere<T>
    {
        fn deserialize<__D>(__d: __D) -> Result<Self, __D::Error>
        where
            __D: serde::Deserializer<'de>,
        {
            #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
            #[expect(
                non_camel_case_types,
                reason = "pg type where requires this localized allowance for generated or framework-constrained code verified by focused tests"
            )]
            #[doc(hidden)]
            enum __Field {
                f0,
                f1,
                __ignore,
            }
            #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
            #[doc(hidden)]
            struct __FieldVisitor;
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    formatter: &mut std::fmt::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(
                        formatter,
                        constants_str::PG_CRUD_FIELD_IDENTIFIER,
                    )
                }
                fn visit_u64<__E>(self, u64: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match u64 {
                        constants_u64::ZERO => Ok(__Field::f0),
                        1u64 => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, str: &str) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match str {
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
                fn deserialize<__D>(__d: __D) -> Result<Self, __D::Error>
                where
                    __D: serde::Deserializer<'de>,
                {
                    serde::Deserializer::deserialize_identifier(__d, __FieldVisitor)
                }
            }
            #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
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
                    formatter: &mut std::fmt::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    std::fmt::Formatter::write_str(
                        formatter,
                        constants_str::PG_CRUD_PG_TYPE_WHERE_STRUCT_NAME,
                    )
                }
                #[inline]
                fn visit_seq<__A>(self, mut __a: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(f0) =
                        _serde::de::SeqAccess::next_element::<crate::operator::Operator>(&mut __a)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            constants_usize::ZERO,
                            &constants_str::PG_CRUD_PG_TYPE_WHERE_EXPECTING,
                        ));
                    };
                    let Some(f1) = _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __a)? else {
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
                fn visit_map<__A>(self, mut __a: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut f0: Option<crate::operator::Operator> = None;
                    let mut f1: Option<Vec<T>> = None;
                    while let Some(__k) = _serde::de::MapAccess::next_key::<__Field>(&mut __a)? {
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
                                >(&mut __a)?);
                            }
                            __Field::f1 => {
                                if Option::is_some(&f1) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            constants_str::PG_CRUD_VALUES_FIELD,
                                        ),
                                    );
                                }
                                f1 = Some(_serde::de::MapAccess::next_value::<Vec<T>>(&mut __a)?);
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny =
                                    _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                        &mut __a,
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
                __d,
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
        sqlx_postgres_query: crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
    ) -> Result<
        crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
        crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    > {
        self.values.into_vec().into_iter().try_fold(
            sqlx_postgres_query,
            |accumulator_query, element| {
                crate::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                    element,
                    accumulator_query,
                )
            },
        )
    }
    fn query_part(
        &self,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
        sql_column_ref: crate::sql_column_ref::SqlColumnRef<'_>,
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
                sql_column_ref,
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
