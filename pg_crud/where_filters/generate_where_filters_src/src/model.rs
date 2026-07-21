#[derive(Clone, Copy)]
#[derive(newtype::FromInner)]
struct BindCount(usize);
#[derive(Clone, Copy, newtype::AsRefInner, newtype::Display, newtype::ToTokens)]
#[derive(newtype::FromInner)]
pub(super) struct FilterSqlOperator(&'static str);
#[derive(Clone, Copy, newtype::AsRefInner, newtype::ToTokens)]
#[derive(newtype::FromInner)]
pub(super) struct FilterSqlSuffix(&'static str);
#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct FilterSpecValid(bool);
impl FilterSpecValid {
    pub(super) const fn get(self) -> bool {
        self.0
    }
}
#[derive(Clone, Copy)]
enum FilterValueShape {
    Scalar,
    Text,
}
#[derive(Clone, Copy)]
pub(super) struct FilterSpec {
    bind_count: BindCount,
    sql_operator: FilterSqlOperator,
    sql_suffix: FilterSqlSuffix,
    value_shape: FilterValueShape,
}
impl FilterSpec {
    pub(super) const ADJACENT: Self = Self::scalar(FilterSqlOperator::from(
        str_constants::PG_CRUD_ADJACENT_SQL_OPERATOR,
    ));
    pub(super) const BEFORE: Self = Self::scalar(FilterSqlOperator::from(
        str_constants::PG_CRUD_BEFORE_SQL_OPERATOR,
    ));
    pub(super) const CONTAINS: Self = Self::scalar(FilterSqlOperator::from(
        str_constants::PG_CRUD_CONTAINS_SQL_OPERATOR,
    ));
    pub(super) const EQUALITY: Self = Self::scalar(FilterSqlOperator::from(
        str_constants::PG_CRUD_EQUALITY_SQL_OPERATOR,
    ));
    pub(super) const LEFT_OF: Self = Self::scalar(FilterSqlOperator::from(
        str_constants::PG_CRUD_LEFT_OF_SQL_OPERATOR,
    ));
    pub(super) const OVERLAPS: Self = Self::scalar(FilterSqlOperator::from(
        str_constants::PG_CRUD_OVERLAPS_SQL_OPERATOR,
    ));
    pub(super) const RIGHT_OF: Self = Self::scalar(FilterSqlOperator::from(
        str_constants::PG_CRUD_RIGHT_OF_SQL_OPERATOR,
    ));
    pub(super) const TEXT_SEARCH: Self = Self {
        bind_count: BindCount::from(1usize),
        sql_operator: FilterSqlOperator::from(str_constants::PG_CRUD_TEXT_SEARCH_SQL_OPERATOR),
        sql_suffix: FilterSqlSuffix::from(str_constants::PG_CRUD_TEXT_SEARCH_SQL_SUFFIX),
        value_shape: FilterValueShape::Text,
    };
    pub(super) const WITHIN: Self = Self::scalar(FilterSqlOperator::from(
        str_constants::PG_CRUD_WITHIN_SQL_OPERATOR,
    ));
    pub(super) const fn bind_count_matches(
        self,
        value: crate::bind::FilterPlaceholderCount,
    ) -> FilterSpecValid {
        FilterSpecValid::from(self.bind_count.0 == value.get())
    }
    pub(super) const fn has_text_value_shape(self) -> FilterSpecValid {
        FilterSpecValid::from(matches!(self.value_shape, FilterValueShape::Text))
    }
    const fn scalar(sql_operator: FilterSqlOperator) -> Self {
        Self {
            bind_count: BindCount::from(1usize),
            sql_operator,
            sql_suffix: FilterSqlSuffix::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            value_shape: FilterValueShape::Scalar,
        }
    }
    pub(super) const fn sql_operator(self) -> FilterSqlOperator {
        self.sql_operator
    }
    pub(super) const fn sql_suffix(self) -> FilterSqlSuffix {
        self.sql_suffix
    }
}
#[cfg(test)]
#[allow(clippy::needless_for_each)] // descriptor matrix avoids repository-forbidden for loops
mod tests {
    #[test]
    fn filter_specs_keep_sql_bind_and_value_shape_in_sync() {
        [
            super::FilterSpec::ADJACENT,
            super::FilterSpec::BEFORE,
            super::FilterSpec::CONTAINS,
            super::FilterSpec::EQUALITY,
            super::FilterSpec::LEFT_OF,
            super::FilterSpec::OVERLAPS,
            super::FilterSpec::RIGHT_OF,
            super::FilterSpec::TEXT_SEARCH,
            super::FilterSpec::WITHIN,
        ]
        .into_iter()
        .for_each(|spec| assert!(crate::contract_tests::filter_spec_contract_is_valid(spec).get()));
    }
}
