#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) struct FilterSpec {
    bind_count: super::BindCount,
    sql_operator: super::FilterSqlOperator,
    sql_suffix: super::FilterSqlSuffix,
    value_shape: super::FilterValueShape,
}

impl FilterSpec {
    pub(crate) fn adjacent() -> Self {
        Self::scalar(super::FilterSqlOperator::from(
            constants_str::PG_CRUD_ADJACENT_SQL_OPERATOR,
        ))
    }
    pub(crate) fn before() -> Self {
        Self::scalar(super::FilterSqlOperator::from(
            constants_str::PG_CRUD_BEFORE_SQL_OPERATOR,
        ))
    }
    pub(crate) fn bind_count_matches(
        self,
        value: crate::domain_types::filter_placeholder_count::FilterPlaceholderCount,
    ) -> super::FilterSpecValid {
        super::FilterSpecValid::from(self.bind_count.0 == value.get())
    }
    pub(crate) fn contains() -> Self {
        Self::scalar(super::FilterSqlOperator::from(
            constants_str::PG_CRUD_CONTAINS_SQL_OPERATOR,
        ))
    }
    pub(crate) fn equality() -> Self {
        Self::scalar(super::FilterSqlOperator::from(
            constants_str::PG_CRUD_EQUALITY_SQL_OPERATOR,
        ))
    }
    pub(crate) fn has_text_value_shape(self) -> super::FilterSpecValid {
        super::FilterSpecValid::from(matches!(self.value_shape, super::FilterValueShape::Text))
    }
    pub(crate) fn left_of() -> Self {
        Self::scalar(super::FilterSqlOperator::from(
            constants_str::PG_CRUD_LEFT_OF_SQL_OPERATOR,
        ))
    }
    pub(crate) fn overlaps() -> Self {
        Self::scalar(super::FilterSqlOperator::from(
            constants_str::PG_CRUD_OVERLAPS_SQL_OPERATOR,
        ))
    }
    pub(crate) fn right_of() -> Self {
        Self::scalar(super::FilterSqlOperator::from(
            constants_str::PG_CRUD_RIGHT_OF_SQL_OPERATOR,
        ))
    }
    fn scalar(sql_operator: super::FilterSqlOperator) -> Self {
        Self {
            bind_count: super::BindCount::from(constants_usize::ONE),
            sql_operator,
            sql_suffix: super::FilterSqlSuffix::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            value_shape: super::FilterValueShape::Scalar,
        }
    }
    pub(crate) const fn sql_operator(self) -> super::FilterSqlOperator {
        self.sql_operator
    }
    pub(crate) const fn sql_suffix(self) -> super::FilterSqlSuffix {
        self.sql_suffix
    }
    pub(crate) fn text_search() -> Self {
        Self {
            bind_count: super::BindCount::from(constants_usize::ONE),
            sql_operator: super::FilterSqlOperator::from(
                constants_str::PG_CRUD_TEXT_SEARCH_SQL_OPERATOR,
            ),
            sql_suffix: super::FilterSqlSuffix::from(constants_str::PG_CRUD_TEXT_SEARCH_SQL_SUFFIX),
            value_shape: super::FilterValueShape::Text,
        }
    }
    pub(crate) fn within() -> Self {
        Self::scalar(super::FilterSqlOperator::from(
            constants_str::PG_CRUD_WITHIN_SQL_OPERATOR,
        ))
    }
}
