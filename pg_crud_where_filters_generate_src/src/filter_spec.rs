#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) struct FilterSpec {
    bind_count: crate::bind_count::BindCount,
    #[getters(copy)]
    sql_operator: crate::filter_sql_operator::FilterSqlOperator,
    #[getters(copy)]
    sql_suffix: crate::filter_sql_suffix::FilterSqlSuffix,
    value_shape: crate::pg_filter_value_shape::PgFilterValueShape,
}

impl FilterSpec {
    pub(crate) fn adjacent() -> Self {
        Self::scalar(crate::filter_sql_operator::FilterSqlOperator::from(
            constants_str::PG_CRUD_ADJACENT_SQL_OPERATOR,
        ))
    }
    pub(crate) fn before() -> Self {
        Self::scalar(crate::filter_sql_operator::FilterSqlOperator::from(
            constants_str::PG_CRUD_BEFORE_SQL_OPERATOR,
        ))
    }
    pub(crate) fn bind_count_matches(
        self,
        value: crate::filter_placeholder_count::FilterPlaceholderCount,
    ) -> crate::filter_spec_valid::FilterSpecValid {
        crate::filter_spec_valid::FilterSpecValid::from(usize::from(self.bind_count) == value.get())
    }
    pub(crate) fn contains() -> Self {
        Self::scalar(crate::filter_sql_operator::FilterSqlOperator::from(
            constants_str::PG_CRUD_CONTAINS_SQL_OPERATOR,
        ))
    }
    pub(crate) fn equality() -> Self {
        Self::scalar(crate::filter_sql_operator::FilterSqlOperator::from(
            constants_str::PG_CRUD_EQUALITY_SQL_OPERATOR,
        ))
    }
    pub(crate) fn has_text_value_shape(self) -> crate::filter_spec_valid::FilterSpecValid {
        crate::filter_spec_valid::FilterSpecValid::from(matches!(
            self.value_shape,
            crate::pg_filter_value_shape::PgFilterValueShape::Text
        ))
    }
    pub(crate) fn left_of() -> Self {
        Self::scalar(crate::filter_sql_operator::FilterSqlOperator::from(
            constants_str::PG_CRUD_LEFT_OF_SQL_OPERATOR,
        ))
    }
    pub(crate) fn overlaps() -> Self {
        Self::scalar(crate::filter_sql_operator::FilterSqlOperator::from(
            constants_str::PG_CRUD_OVERLAPS_SQL_OPERATOR,
        ))
    }
    pub(crate) fn right_of() -> Self {
        Self::scalar(crate::filter_sql_operator::FilterSqlOperator::from(
            constants_str::PG_CRUD_RIGHT_OF_SQL_OPERATOR,
        ))
    }
    fn scalar(sql_operator: crate::filter_sql_operator::FilterSqlOperator) -> Self {
        Self {
            bind_count: crate::bind_count::BindCount::from(constants_usize::ONE),
            sql_operator,
            sql_suffix: crate::filter_sql_suffix::FilterSqlSuffix::from(
                constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            ),
            value_shape: crate::pg_filter_value_shape::PgFilterValueShape::Scalar,
        }
    }

    pub(crate) fn text_search() -> Self {
        Self {
            bind_count: crate::bind_count::BindCount::from(constants_usize::ONE),
            sql_operator: crate::filter_sql_operator::FilterSqlOperator::from(
                constants_str::PG_CRUD_TEXT_SEARCH_SQL_OPERATOR,
            ),
            sql_suffix: crate::filter_sql_suffix::FilterSqlSuffix::from(
                constants_str::PG_CRUD_TEXT_SEARCH_SQL_SUFFIX,
            ),
            value_shape: crate::pg_filter_value_shape::PgFilterValueShape::Text,
        }
    }
    pub(crate) fn within() -> Self {
        Self::scalar(crate::filter_sql_operator::FilterSqlOperator::from(
            constants_str::PG_CRUD_WITHIN_SQL_OPERATOR,
        ))
    }
}
