use super::domain_types::{BoundedVecLen, BoundedVecTryNewError, Variant};

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsSlice,
    newtype::IntoInner,
)]
pub struct BoundedVec<T, const LENGTH: usize>(Vec<T>);
impl<T, const LENGTH: usize> From<[T; LENGTH]> for BoundedVec<T, LENGTH> {
    fn from(value: [T; LENGTH]) -> Self {
        Self(Vec::from(value))
    }
}
impl<
    'lt,
    T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + 'lt,
    const LENGTH: usize,
> BoundedVec<T, LENGTH>
{
    pub fn pg_type_query_part(
        &self,
        increment: &mut dyn pg_crud_common::domain_types::QueryPartIncrementMut,
        column: pg_crud_common::domain_types::SqlColumnRef<'_>,
        add_operator: pg_crud_common::domain_types::AddOperator,
    ) -> Result<
        pg_crud_common::domain_types::QueryPartFragment,
        pg_crud_common::domain_types::QueryPartError,
    > {
        self.query_part(increment, column, add_operator, &Variant::Normal)
    }
    pub fn pg_type_query_part_minus_one(
        &self,
        increment: &mut dyn pg_crud_common::domain_types::QueryPartIncrementMut,
        column: pg_crud_common::domain_types::SqlColumnRef<'_>,
        add_operator: pg_crud_common::domain_types::AddOperator,
    ) -> Result<
        pg_crud_common::domain_types::QueryPartFragment,
        pg_crud_common::domain_types::QueryPartError,
    > {
        self.query_part(increment, column, add_operator, &Variant::MinusOne)
    }
    pub fn query_bind(
        self,
        query: pg_crud_common::domain_types::SqlxPostgresQuery<'lt>,
    ) -> Result<
        pg_crud_common::domain_types::SqlxPostgresQuery<'lt>,
        pg_crud_common::domain_types::SqlxPostgresQueryBindError,
    > {
        self.0
            .into_iter()
            .try_fold(query, |mut accumulator_query, element| {
                accumulator_query
                    .as_mut()
                    .try_bind(element)
                    .map_err(pg_crud_common::domain_types::SqlxPostgresQueryBindError::from)?;
                Ok(accumulator_query)
            })
    }
    fn query_part(
        &self,
        increment: &mut dyn pg_crud_common::domain_types::QueryPartIncrementMut,
        _: pg_crud_common::domain_types::SqlColumnRef<'_>,
        _add_operator: pg_crud_common::domain_types::AddOperator,
        variant: &Variant,
    ) -> Result<
        pg_crud_common::domain_types::QueryPartFragment,
        pg_crud_common::domain_types::QueryPartError,
    > {
        let len = match &variant {
            Variant::MinusOne => self.0.len().saturating_sub(1),
            Variant::Normal => self.0.len(),
        };
        let mut accumulator = String::with_capacity(len.saturating_mul(8));
        (0..len).try_for_each(|_| {
            let v =
                match pg_crud_common::domain_types::increment_checked_add_one_returning_increment(
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
                    pg_crud_common::domain_types::QueryPartError::WriteIntoBuffer {
                        location: location_macros::location!(),
                    },
                );
            }
            Ok::<(), pg_crud_common::domain_types::QueryPartError>(())
        })?;
        Ok(pg_crud_common::domain_types::QueryPartFragment::try_from(
            accumulator,
        )?)
    }
}
impl<T, const LENGTH: usize> TryFrom<Vec<T>> for BoundedVec<T, LENGTH> {
    type Error = BoundedVecTryNewError;
    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        let len = v.len();
        bounded_types::BoundedVec::<T, LENGTH, LENGTH>::try_from(v)
            .map(bounded_types::BoundedVec::into_inner)
            .map(Self)
            .map_err(|_error| BoundedVecTryNewError::LenIsNotCorrect {
                wrong_len: BoundedVecLen::from(len),
                expected: BoundedVecLen::from(LENGTH),
                location: location_macros::location!(),
            })
    }
}
impl<'de, T: serde::Deserialize<'de>, const LENGTH: usize> serde::Deserialize<'de>
    for BoundedVec<T, LENGTH>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value =
            <bounded_types::BoundedVec<T, LENGTH, LENGTH> as serde::Deserialize>::deserialize(
                deserializer,
            )?
            .into_inner();
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl<T: Clone + pg_crud_common::domain_types::DefaultSomeOneElement, const LENGTH: usize>
    pg_crud_common::domain_types::DefaultSomeOneElement for BoundedVec<T, LENGTH>
{
    fn default_some_one_element() -> Self {
        Self::from(std::array::from_fn(|_| {
            <T as pg_crud_common::domain_types::DefaultSomeOneElement>::default_some_one_element()
        }))
    }
}
