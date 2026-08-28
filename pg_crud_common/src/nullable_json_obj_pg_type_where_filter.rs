use crate::{
    AddOperator, AllEnumVariants, AllEnumVariantsArrayDefaultSomeOneElement, DefaultSomeOneElement,
    NotEmptyUniqueVec, PgTypeWhereFilter, QueryPartError, QueryPartFragment, QueryPartIncrementMut,
    SqlColumnRef, SqlxPostgresQuery, SqlxPostgresQueryBindError,
};

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
#[serde(from = "Option<NotEmptyUniqueVec<T>>")]
pub struct NullableJsonObjPgTypeWhereFilter<
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'lt> PgTypeWhereFilter<'lt>
        + AllEnumVariantsArrayDefaultSomeOneElement,
>(Option<NotEmptyUniqueVec<T>>);
impl<T> NullableJsonObjPgTypeWhereFilter<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> PgTypeWhereFilter<'t_lt>
        + AllEnumVariantsArrayDefaultSomeOneElement,
{
    #[must_use]
    pub const fn as_ref(&self) -> Option<&NotEmptyUniqueVec<T>> {
        self.0.as_ref()
    }
    #[must_use]
    pub fn into_option(self) -> Option<NotEmptyUniqueVec<T>> {
        self.0
    }
}
impl<'query_lt, T> PgTypeWhereFilter<'query_lt> for NullableJsonObjPgTypeWhereFilter<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> PgTypeWhereFilter<'t_lt>
        + AllEnumVariantsArrayDefaultSomeOneElement,
{
    fn query_bind(
        self,
        query: SqlxPostgresQuery<'query_lt>,
    ) -> Result<SqlxPostgresQuery<'query_lt>, SqlxPostgresQueryBindError> {
        match self.into_option() {
            Some(v) => v.query_bind(query),
            None => Ok(query), //todo maybe wrong
        }
    }
    fn query_part(
        &self,
        increment: &mut dyn QueryPartIncrementMut,
        column: SqlColumnRef<'_>,
        add_operator: AddOperator,
    ) -> Result<QueryPartFragment, QueryPartError> {
        self.as_ref().map_or_else(
            || {
                let mut query_part = String::with_capacity(16);
                if std::fmt::Write::write_fmt(&mut query_part, format_args!("{column} = 'null'"))
                    .is_err()
                {
                    return Err(QueryPartError::WriteIntoBuffer {
                        location: location_macros::location!(),
                    });
                }
                Ok(QueryPartFragment::try_from(query_part).unwrap_or_else(QueryPartFragment::from))
            },
            |v| v.query_part(increment, column, add_operator),
        )
    }
}
impl<T> to_err_string::domain_types::ToErrString for NullableJsonObjPgTypeWhereFilter<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> PgTypeWhereFilter<'t_lt>
        + AllEnumVariantsArrayDefaultSomeOneElement,
{
    fn to_err_string(&self) -> to_err_string::domain_types::ErrorText {
        to_err_string::domain_types::ErrorText::try_from(format!("{self:#?}"))
            .unwrap_or_else(to_err_string::domain_types::ErrorText::from)
    }
}
impl<T> AllEnumVariantsArrayDefaultSomeOneElement for NullableJsonObjPgTypeWhereFilter<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> PgTypeWhereFilter<'t_lt>
        + AllEnumVariantsArrayDefaultSomeOneElement,
{
    fn all_variants_default_some_one_element() -> AllEnumVariants<Self> {
        vec![Self(
            Some(DefaultSomeOneElement::default_some_one_element()),
        )]
        .into()
    }
}
