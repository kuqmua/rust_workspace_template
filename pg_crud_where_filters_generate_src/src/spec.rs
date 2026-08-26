#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct BindCount(usize);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::Display,
    newtype::ToTokens,
    newtype::FromInner,
)]
pub(super) struct FilterSqlOperator(&'static str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::ToTokens,
    newtype::FromInner,
)]
pub(super) struct FilterSqlSuffix(&'static str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct FilterSpecValid(bool);
impl FilterSpecValid {
    pub(super) const fn get(self) -> bool {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
enum FilterValueShape {
    Scalar,
    Text,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(super) struct FilterSpec {
    bind_count: BindCount,
    sql_operator: FilterSqlOperator,
    sql_suffix: FilterSqlSuffix,
    value_shape: FilterValueShape,
}
impl FilterSpec {
    pub(super) fn adjacent() -> Self {
        Self::scalar(FilterSqlOperator::from(
            constants_str::PG_CRUD_ADJACENT_SQL_OPERATOR,
        ))
    }
    pub(super) fn before() -> Self {
        Self::scalar(FilterSqlOperator::from(
            constants_str::PG_CRUD_BEFORE_SQL_OPERATOR,
        ))
    }
    pub(super) fn bind_count_matches(
        self,
        value: crate::domain_types::bind::FilterPlaceholderCount,
    ) -> FilterSpecValid {
        FilterSpecValid::from(self.bind_count.0 == value.get())
    }
    pub(super) fn contains() -> Self {
        Self::scalar(FilterSqlOperator::from(
            constants_str::PG_CRUD_CONTAINS_SQL_OPERATOR,
        ))
    }
    pub(super) fn equality() -> Self {
        Self::scalar(FilterSqlOperator::from(
            constants_str::PG_CRUD_EQUALITY_SQL_OPERATOR,
        ))
    }
    pub(super) fn has_text_value_shape(self) -> FilterSpecValid {
        FilterSpecValid::from(matches!(self.value_shape, FilterValueShape::Text))
    }
    pub(super) fn left_of() -> Self {
        Self::scalar(FilterSqlOperator::from(
            constants_str::PG_CRUD_LEFT_OF_SQL_OPERATOR,
        ))
    }
    pub(super) fn overlaps() -> Self {
        Self::scalar(FilterSqlOperator::from(
            constants_str::PG_CRUD_OVERLAPS_SQL_OPERATOR,
        ))
    }
    pub(super) fn right_of() -> Self {
        Self::scalar(FilterSqlOperator::from(
            constants_str::PG_CRUD_RIGHT_OF_SQL_OPERATOR,
        ))
    }
    fn scalar(sql_operator: FilterSqlOperator) -> Self {
        Self {
            bind_count: BindCount::from(constants_usize::ONE),
            sql_operator,
            sql_suffix: FilterSqlSuffix::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            value_shape: FilterValueShape::Scalar,
        }
    }
    pub(super) const fn sql_operator(self) -> FilterSqlOperator {
        self.sql_operator
    }
    pub(super) const fn sql_suffix(self) -> FilterSqlSuffix {
        self.sql_suffix
    }
    pub(super) fn text_search() -> Self {
        Self {
            bind_count: BindCount::from(constants_usize::ONE),
            sql_operator: FilterSqlOperator::from(constants_str::PG_CRUD_TEXT_SEARCH_SQL_OPERATOR),
            sql_suffix: FilterSqlSuffix::from(constants_str::PG_CRUD_TEXT_SEARCH_SQL_SUFFIX),
            value_shape: FilterValueShape::Text,
        }
    }
    pub(super) fn within() -> Self {
        Self::scalar(FilterSqlOperator::from(
            constants_str::PG_CRUD_WITHIN_SQL_OPERATOR,
        ))
    }
}
#[cfg(test)]
#[allow(clippy::needless_for_each)] // descriptor matrix avoids repository-forbidden for loops
mod tests {
    #[test]
    fn filter_specs_keep_sql_bind_and_value_shape_in_sync() {
        [
            super::FilterSpec::adjacent(),
            super::FilterSpec::before(),
            super::FilterSpec::contains(),
            super::FilterSpec::equality(),
            super::FilterSpec::left_of(),
            super::FilterSpec::overlaps(),
            super::FilterSpec::right_of(),
            super::FilterSpec::text_search(),
            super::FilterSpec::within(),
        ]
        .into_iter()
        .for_each(|spec| {
            assert!(
                crate::domain_types::filter_spec_contract_is_valid::filter_spec_contract_is_valid(
                    spec
                )
                .get()
            );
        });
    }
}
