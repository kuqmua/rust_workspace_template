#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySortOrder {
    Ascending,
    Descending,
}
impl QuerySortOrder {
    const fn sql(self) -> SqlSortOrderText {
        SqlSortOrderText(match self {
            Self::Ascending => str_constants::SORT_ASC,
            Self::Descending => str_constants::SORT_DESC,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SqlSortOrderText(&'static str);
impl AsRef<str> for SqlSortOrderText {
    fn as_ref(&self) -> &str {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdReadQueryBindIndex(std::num::NonZeroU32);
impl From<std::num::NonZeroU32> for StdReadQueryBindIndex {
    fn from(value: std::num::NonZeroU32) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadQueryPlan(crate::QueryPartFragment);
impl From<ReadQueryPlan> for crate::QueryPartFragment {
    fn from(value: ReadQueryPlan) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("read query plan exceeds the query fragment limit")]
pub struct ReadQueryPlanError;

pub fn build_stable_read_query_plan(
    base: crate::QueryPartFragment,
    sort_column: &crate::SqlIdentifier,
    tie_break_column: &crate::SqlIdentifier,
    order: QuerySortOrder,
    limit_bind: StdReadQueryBindIndex,
    offset_bind: StdReadQueryBindIndex,
) -> Result<ReadQueryPlan, ReadQueryPlanError> {
    let mut query = base.into_inner();
    query.push_str(str_constants::READ_ORDER_BY);
    query.push_str(sort_column.as_ref());
    query.push(' ');
    query.push_str(order.sql().as_ref());
    if sort_column != tie_break_column {
        query.push_str(str_constants::TEXT_ALT_6);
        query.push_str(tie_break_column.as_ref());
        query.push(' ');
        query.push_str(order.sql().as_ref());
    }
    query.push_str(str_constants::LIMIT_DOLLAR);
    std::fmt::Write::write_fmt(&mut query, format_args!("{}", limit_bind.0))
        .map_err(|_error| ReadQueryPlanError)?;
    query.push_str(str_constants::OFFSET_DOLLAR);
    std::fmt::Write::write_fmt(&mut query, format_args!("{}", offset_bind.0))
        .map_err(|_error| ReadQueryPlanError)?;
    crate::QueryPartFragment::try_from(query)
        .map(ReadQueryPlan)
        .map_err(|_error| ReadQueryPlanError)
}

#[cfg(test)]
mod tests {
    fn identifier(value: &str) -> crate::SqlIdentifier {
        crate::SqlIdentifier::try_from(value.to_owned()).expect("cd7c83ed")
    }

    #[test]
    fn stable_plan_appends_tie_break_limit_and_offset() {
        let plan = super::build_stable_read_query_plan(
            crate::QueryPartFragment::try_from(String::from(str_constants::TEST_READ_QUERY_BASE))
                .expect("ef7cd3e2"),
            &identifier(str_constants::CREATED_AT),
            &identifier(str_constants::SQL_NAMES_ID),
            super::QuerySortOrder::Descending,
            std::num::NonZeroU32::new(1u32).expect("2c810064").into(),
            std::num::NonZeroU32::new(2u32).expect("aa77f541").into(),
        )
        .expect("377c56d0");
        let fragment = crate::QueryPartFragment::from(plan);
        assert_eq!(fragment.into_inner(), str_constants::TEST_STABLE_READ_QUERY);
    }
}
