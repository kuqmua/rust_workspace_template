#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct PaginationBase {
    limit: crate::pagination_limit::PaginationLimit,
    offset: crate::pagination_offset::PaginationOffset,
}

impl PaginationBase {
    #[must_use]
    pub fn end(&self) -> crate::pagination_end::PaginationEnd {
        crate::pagination_end::PaginationEnd::from(
            self.offset.get().saturating_add(self.limit.get()),
        )
    }

    #[must_use]
    pub fn new_unchecked<LimitTy, OffsetTy>(limit: LimitTy, offset: OffsetTy) -> Self
    where
        LimitTy: Into<crate::pagination_limit::PaginationLimit>,
        OffsetTy: Into<crate::pagination_offset::PaginationOffset>,
    {
        Self {
            limit: limit.into(),
            offset: offset.into(),
        }
    }

    #[must_use]
    pub fn start(&self) -> crate::pagination_start::PaginationStart {
        crate::pagination_start::PaginationStart::from(self.offset.get())
    }
}

impl<'query_lt> crate::pg_type_where_filter::PgTypeWhereFilter<'query_lt> for PaginationBase {
    fn query_bind(
        self,
        mut query: crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
    ) -> Result<
        crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
        crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    > {
        if let Err(error) = query.as_mut().try_bind(self.limit.get()) {
            return Err(
                crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError::from(error),
            );
        }
        if let Err(error) = query.as_mut().try_bind(self.offset.get()) {
            return Err(
                crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError::from(error),
            );
        }
        Ok(query)
    }

    fn query_part(
        &self,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
        _: crate::sql_column_ref::SqlColumnRef<'_>,
        _: crate::add_operator::AddOperator,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    > {
        let limit_increment =
            crate::increment_checked_add_one_returning_increment::increment_checked_add_one_returning_increment(increment)?;
        let offset_increment =
            crate::increment_checked_add_one_returning_increment::increment_checked_add_one_returning_increment(increment)?;
        let mut query_part = String::with_capacity(32);
        if std::fmt::Write::write_fmt(
            &mut query_part,
            format_args!("limit ${limit_increment} offset ${offset_increment}"),
        )
        .is_err()
        {
            return Err(crate::query_part_error::QueryPartError::WriteIntoBuffer {
                location: location_macros::location!(),
            });
        }
        Ok(crate::query_part_fragment::QueryPartFragment::try_from(
            query_part,
        )?)
    }
}

impl Default for PaginationBase {
    fn default() -> Self {
        Self::new_unchecked(
            crate::pagination_policy::PaginationPolicy::standard()
                .default_limit()
                .get(),
            0,
        )
    }
}
