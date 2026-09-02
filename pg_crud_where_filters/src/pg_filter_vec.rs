#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    schemars::JsonSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::AsSlice,
    proc_macro_newtype::IntoInner,
)]
pub struct PgFilterVec<T, const LENGTH: usize>(Vec<T>);
impl<T, const LENGTH: usize> From<[T; LENGTH]> for PgFilterVec<T, LENGTH> {
    fn from(value: [T; LENGTH]) -> Self {
        Self(Vec::from(value))
    }
}
impl<
    'lt,
    T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + 'lt,
    const LENGTH: usize,
> PgFilterVec<T, LENGTH>
{
    pub fn pg_type_query_part(
        &self,
        increment: &mut dyn pg_crud_common::query_part_increment_mut::QueryPartIncrementMut,
        sql_column_ref: pg_crud_common::sql_column_ref::SqlColumnRef<'_>,
        add_operator: pg_crud_common::add_operator::AddOperator,
    ) -> Result<
        pg_crud_common::query_part_fragment::QueryPartFragment,
        pg_crud_common::query_part_error::QueryPartError,
    > {
        self.query_part(
            increment,
            sql_column_ref,
            add_operator,
            &crate::variant::Variant::Normal,
        )
    }
    pub fn pg_type_query_part_minus_one(
        &self,
        increment: &mut dyn pg_crud_common::query_part_increment_mut::QueryPartIncrementMut,
        sql_column_ref: pg_crud_common::sql_column_ref::SqlColumnRef<'_>,
        add_operator: pg_crud_common::add_operator::AddOperator,
    ) -> Result<
        pg_crud_common::query_part_fragment::QueryPartFragment,
        pg_crud_common::query_part_error::QueryPartError,
    > {
        self.query_part(
            increment,
            sql_column_ref,
            add_operator,
            &crate::variant::Variant::MinusOne,
        )
    }
    pub fn query_bind(
        self,
        sqlx_postgres_query: pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery<'lt>,
    ) -> Result<
        pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery<'lt>,
        pg_crud_common::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    > {
        self.0
            .into_iter()
            .try_fold(sqlx_postgres_query, |mut accumulator_query, element| {
                accumulator_query
                    .as_mut()
                    .try_bind(element)
                    .map_err(pg_crud_common::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError::from)?;
                Ok(accumulator_query)
            })
    }
    #[allow(
        unused_variables,
        reason = "the query contract preserves repository type-based parameter names"
    )]
    fn query_part(
        &self,
        increment: &mut dyn pg_crud_common::query_part_increment_mut::QueryPartIncrementMut,
        sql_column_ref: pg_crud_common::sql_column_ref::SqlColumnRef<'_>,
        add_operator: pg_crud_common::add_operator::AddOperator,
        variant: &crate::variant::Variant,
    ) -> Result<
        pg_crud_common::query_part_fragment::QueryPartFragment,
        pg_crud_common::query_part_error::QueryPartError,
    > {
        let len = match &variant {
            crate::variant::Variant::MinusOne => self.0.len().saturating_sub(1),
            crate::variant::Variant::Normal => self.0.len(),
        };
        let mut accumulator = String::with_capacity(len.saturating_mul(8));
        (0..len).try_for_each(|_| {
            let v =
                match pg_crud_common::increment_checked_add_one_returning_increment::increment_checked_add_one_returning_increment(
                    increment,
                ) {
                    Ok(v) => v,
                    Err(error) => {
                        return Err(error);
                    }
                };
            let write_res = std::fmt::Write::write_fmt(&mut accumulator, format_args!("[${v}]"));
            if write_res.is_err() {
                return Err(
                    pg_crud_common::query_part_error::QueryPartError::WriteIntoBuffer {
                        location: proc_macro_location_bang::location!(),
                    },
                );
            }
            Ok::<(), pg_crud_common::query_part_error::QueryPartError>(())
        })?;
        Ok(pg_crud_common::query_part_fragment::QueryPartFragment::try_from(accumulator)?)
    }
}
impl<T, const LENGTH: usize> TryFrom<Vec<T>> for PgFilterVec<T, LENGTH> {
    type Error = crate::bounded_vec_try_new_error::BoundedVecTryNewError;
    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        let len = vec.len();
        bounded_types::bounded_vec::BoundedVec::<T, LENGTH, LENGTH>::try_from(vec)
            .map(bounded_types::bounded_vec::BoundedVec::into_inner)
            .map(Self)
            .map_err(|_error| {
                crate::bounded_vec_try_new_error::BoundedVecTryNewError::LenIsNotCorrect {
                    wrong_len: crate::pg_filter_vec_len::PgFilterVecLen::from(len),
                    expected: crate::pg_filter_vec_len::PgFilterVecLen::from(LENGTH),
                    location: proc_macro_location_bang::location!(),
                }
            })
    }
}
impl<'de, T: serde::Deserialize<'de>, const LENGTH: usize> serde::Deserialize<'de>
    for PgFilterVec<T, LENGTH>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value =
            <bounded_types::bounded_vec::BoundedVec<T, LENGTH, LENGTH> as serde::Deserialize>::deserialize(
                deserializer,
            )?
            .into_inner();
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl<
    T: Clone + pg_crud_common::default_some_one_element::DefaultSomeOneElement,
    const LENGTH: usize,
> pg_crud_common::default_some_one_element::DefaultSomeOneElement for PgFilterVec<T, LENGTH>
{
    fn default_some_one_element() -> Self {
        Self::from(std::array::from_fn(|_| {
            <T as pg_crud_common::default_some_one_element::DefaultSomeOneElement>::default_some_one_element()
        }))
    }
}
