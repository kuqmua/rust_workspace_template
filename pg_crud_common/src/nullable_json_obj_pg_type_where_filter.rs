#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
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
        sqlx_postgres_query: crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
    ) -> Result<crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>, crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError> {
        match self.into_option() {
            Some(v) => v.query_bind(sqlx_postgres_query),
            None => Ok(sqlx_postgres_query),
        }
    }
    fn query_part(
        &self,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
        sql_column_ref: crate::sql_column_ref::SqlColumnRef<'_>,
        add_operator: crate::add_operator::AddOperator,
    ) -> Result<crate::query_part_fragment::QueryPartFragment, crate::query_part_error::QueryPartError> {
        self.as_ref().map_or_else(
            || {
                let mut query_part = String::with_capacity(16);
                if std::fmt::Write::write_fmt(&mut query_part, format_args!("{sql_column_ref} = 'null'"))
                    .is_err()
                {
                    return Err(crate::query_part_error::QueryPartError::WriteIntoBuffer {
                        location: proc_macro_location_bang::location!(),
                    });
                }
                Ok(crate::query_part_fragment::QueryPartFragment::try_from(query_part)
                    .unwrap_or_else(crate::query_part_fragment::QueryPartFragment::from))
            },
            |v| v.query_part(increment, sql_column_ref, add_operator),
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
