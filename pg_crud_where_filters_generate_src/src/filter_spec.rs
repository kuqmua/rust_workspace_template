#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) struct FilterSpec {
    bind_count: crate::spec::BindCount,
    sql_operator: crate::spec::FilterSqlOperator,
    sql_suffix: crate::spec::FilterSqlSuffix,
    value_shape: crate::spec::FilterValueShape,
}

impl FilterSpec {
    pub(crate) fn adjacent() -> Self {
        Self::scalar(crate::spec::FilterSqlOperator::from(
            constants_str::PG_CRUD_ADJACENT_SQL_OPERATOR,
        ))
    }
    pub(crate) fn before() -> Self {
        Self::scalar(crate::spec::FilterSqlOperator::from(
            constants_str::PG_CRUD_BEFORE_SQL_OPERATOR,
        ))
    }
    pub(crate) fn bind_count_matches(
        self,
        value: crate::filter_placeholder_count::FilterPlaceholderCount,
    ) -> crate::spec::FilterSpecValid {
        crate::spec::FilterSpecValid::from(self.bind_count.0 == value.get())
    }
    pub(crate) fn contains() -> Self {
        Self::scalar(crate::spec::FilterSqlOperator::from(
            constants_str::PG_CRUD_CONTAINS_SQL_OPERATOR,
        ))
    }
    pub(crate) fn equality() -> Self {
        Self::scalar(crate::spec::FilterSqlOperator::from(
            constants_str::PG_CRUD_EQUALITY_SQL_OPERATOR,
        ))
    }
    pub(crate) fn has_text_value_shape(self) -> crate::spec::FilterSpecValid {
        crate::spec::FilterSpecValid::from(matches!(
            self.value_shape,
            crate::spec::FilterValueShape::Text
        ))
    }
    pub(crate) fn left_of() -> Self {
        Self::scalar(crate::spec::FilterSqlOperator::from(
            constants_str::PG_CRUD_LEFT_OF_SQL_OPERATOR,
        ))
    }
    pub(crate) fn overlaps() -> Self {
        Self::scalar(crate::spec::FilterSqlOperator::from(
            constants_str::PG_CRUD_OVERLAPS_SQL_OPERATOR,
        ))
    }
    pub(crate) fn right_of() -> Self {
        Self::scalar(crate::spec::FilterSqlOperator::from(
            constants_str::PG_CRUD_RIGHT_OF_SQL_OPERATOR,
        ))
    }
    fn scalar(sql_operator: crate::spec::FilterSqlOperator) -> Self {
        Self {
            bind_count: crate::spec::BindCount::from(constants_usize::ONE),
            sql_operator,
            sql_suffix: crate::spec::FilterSqlSuffix::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            value_shape: crate::spec::FilterValueShape::Scalar,
        }
    }
    pub(crate) const fn sql_operator(self) -> crate::spec::FilterSqlOperator {
        self.sql_operator
    }
    pub(crate) const fn sql_suffix(self) -> crate::spec::FilterSqlSuffix {
        self.sql_suffix
    }
    pub(crate) fn text_search() -> Self {
        Self {
            bind_count: crate::spec::BindCount::from(constants_usize::ONE),
            sql_operator: crate::spec::FilterSqlOperator::from(
                constants_str::PG_CRUD_TEXT_SEARCH_SQL_OPERATOR,
            ),
            sql_suffix: crate::spec::FilterSqlSuffix::from(
                constants_str::PG_CRUD_TEXT_SEARCH_SQL_SUFFIX,
            ),
            value_shape: crate::spec::FilterValueShape::Text,
        }
    }
    pub(crate) fn within() -> Self {
        Self::scalar(crate::spec::FilterSqlOperator::from(
            constants_str::PG_CRUD_WITHIN_SQL_OPERATOR,
        ))
    }
}
