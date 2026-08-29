//todo custom deserialization - must not contain more than one element
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
)]
#[serde(from = "Option<crate::not_empty_unique_vec::NotEmptyUniqueVec<T>>")]
pub struct NullableJsonObjPgTypeWhereFilter<
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'lt> crate::pg_type_where_filter::PgTypeWhereFilter<'lt>
        + crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement,
>(Option<crate::not_empty_unique_vec::NotEmptyUniqueVec<T>>);
impl<T> NullableJsonObjPgTypeWhereFilter<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> crate::pg_type_where_filter::PgTypeWhereFilter<'t_lt>
        + crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement,
{
    #[must_use]
    pub const fn as_ref(&self) -> Option<&crate::not_empty_unique_vec::NotEmptyUniqueVec<T>> {
        self.0.as_ref()
    }
    #[must_use]
    pub fn into_option(self) -> Option<crate::not_empty_unique_vec::NotEmptyUniqueVec<T>> {
        self.0
    }
}
impl<'query_lt, T> crate::pg_type_where_filter::PgTypeWhereFilter<'query_lt> for NullableJsonObjPgTypeWhereFilter<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> crate::pg_type_where_filter::PgTypeWhereFilter<'t_lt>
        + crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement,
{
    fn query_bind(
        self,
        query: crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
    ) -> Result<crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>, crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError> {
        match self.into_option() {
            Some(v) => v.query_bind(query),
            None => Ok(query), //todo maybe wrong
        }
    }
    fn query_part(
        &self,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
        column: crate::sql_column_ref::SqlColumnRef<'_>,
        add_operator: crate::add_operator::AddOperator,
    ) -> Result<crate::query_part_fragment::QueryPartFragment, crate::query_part_error::QueryPartError> {
        self.as_ref().map_or_else(
            || {
                let mut query_part = String::with_capacity(16);
                if std::fmt::Write::write_fmt(&mut query_part, format_args!("{column} = 'null'"))
                    .is_err()
                {
                    return Err(crate::query_part_error::QueryPartError::WriteIntoBuffer {
                        location: location_macros::location!(),
                    });
                }
                Ok(crate::query_part_fragment::QueryPartFragment::try_from(query_part)
                    .unwrap_or_else(crate::query_part_fragment::QueryPartFragment::from))
            },
            |v| v.query_part(increment, column, add_operator),
        )
    }
}
impl<T> to_err_string::to_err_string::ToErrString for NullableJsonObjPgTypeWhereFilter<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> crate::pg_type_where_filter::PgTypeWhereFilter<'t_lt>
        + crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement,
{
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(format!("{self:#?}"))
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}
impl<T> crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement for NullableJsonObjPgTypeWhereFilter<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> crate::pg_type_where_filter::PgTypeWhereFilter<'t_lt>
        + crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement,
{
    fn all_variants_default_some_one_element() -> crate::all_enum_variants::AllEnumVariants<Self> {
        vec![Self(Some(
            crate::default_some_one_element::DefaultSomeOneElement::default_some_one_element(),
        ))]
        .into()
    }
}
