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
    limit: crate::domain_types::PaginationLimit,
    offset: crate::domain_types::PaginationOffset,
}

impl PaginationBase {
    #[must_use]
    pub fn end(&self) -> crate::domain_types::PaginationEnd {
        crate::domain_types::PaginationEnd::from(self.offset.get().saturating_add(self.limit.get()))
    }

    #[must_use]
    pub fn new_unchecked<LimitTy, OffsetTy>(limit: LimitTy, offset: OffsetTy) -> Self
    where
        LimitTy: Into<crate::domain_types::PaginationLimit>,
        OffsetTy: Into<crate::domain_types::PaginationOffset>,
    {
        Self {
            limit: limit.into(),
            offset: offset.into(),
        }
    }

    #[must_use]
    pub fn start(&self) -> crate::domain_types::PaginationStart {
        crate::domain_types::PaginationStart::from(self.offset.get())
    }
}

impl<'query_lt> crate::domain_types::PgTypeWhereFilter<'query_lt> for PaginationBase {
    fn query_bind(
        self,
        mut query: crate::domain_types::SqlxPostgresQuery<'query_lt>,
    ) -> Result<
        crate::domain_types::SqlxPostgresQuery<'query_lt>,
        crate::domain_types::SqlxPostgresQueryBindError,
    > {
        if let Err(error) = query.as_mut().try_bind(self.limit.get()) {
            return Err(crate::domain_types::SqlxPostgresQueryBindError::from(error));
        }
        if let Err(error) = query.as_mut().try_bind(self.offset.get()) {
            return Err(crate::domain_types::SqlxPostgresQueryBindError::from(error));
        }
        Ok(query)
    }

    fn query_part(
        &self,
        increment: &mut dyn crate::domain_types::QueryPartIncrementMut,
        _: crate::domain_types::SqlColumnRef<'_>,
        _: crate::domain_types::AddOperator,
    ) -> Result<crate::domain_types::QueryPartFragment, crate::domain_types::QueryPartError> {
        let limit_increment =
            match crate::domain_types::increment_checked_add_one_returning_increment(increment) {
                Ok(v) => v,
                Err(error) => return Err(error),
            };
        let offset_increment =
            match crate::domain_types::increment_checked_add_one_returning_increment(increment) {
                Ok(v) => v,
                Err(error) => return Err(error),
            };
        let mut query_part = String::with_capacity(32);
        if std::fmt::Write::write_fmt(
            &mut query_part,
            format_args!("limit ${limit_increment} offset ${offset_increment}"),
        )
        .is_err()
        {
            return Err(crate::domain_types::QueryPartError::WriteIntoBuffer {
                location: location_macros::location!(),
            });
        }
        Ok(crate::domain_types::QueryPartFragment::try_from(
            query_part,
        )?)
    }
}

impl Default for PaginationBase {
    fn default() -> Self {
        Self::new_unchecked(
            crate::domain_types::PaginationPolicy::standard()
                .default_limit()
                .get(),
            0,
        )
    }
}

#[cfg(test)]
#[path = "domain_types_query_pagination_tests.rs"]
mod tests;
