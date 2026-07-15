#[derive(Clone, Copy)]
struct BindCount(usize);
#[derive(Clone, Copy)]
pub(super) struct FilterSqlOperator(&'static str);
impl AsRef<str> for FilterSqlOperator {
    fn as_ref(&self) -> &str {
        self.0
    }
}
impl std::fmt::Display for FilterSqlOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl quote::ToTokens for FilterSqlOperator {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
#[derive(Clone, Copy)]
pub(super) struct FilterSqlSuffix(&'static str);
impl AsRef<str> for FilterSqlSuffix {
    fn as_ref(&self) -> &str {
        self.0
    }
}
impl quote::ToTokens for FilterSqlSuffix {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
#[derive(Clone, Copy)]
pub(super) struct FilterSpecValid(bool);
impl From<bool> for FilterSpecValid {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
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
    pub(super) const ADJACENT: Self = Self::scalar(FilterSqlOperator(
        str_constants::pg_crud::ADJACENT_SQL_OPERATOR,
    ));
    pub(super) const BEFORE: Self = Self::scalar(FilterSqlOperator(
        str_constants::pg_crud::BEFORE_SQL_OPERATOR,
    ));
    pub(super) const CONTAINS: Self = Self::scalar(FilterSqlOperator(
        str_constants::pg_crud::CONTAINS_SQL_OPERATOR,
    ));
    pub(super) const EQUALITY: Self = Self::scalar(FilterSqlOperator(
        str_constants::pg_crud::EQUALITY_SQL_OPERATOR,
    ));
    pub(super) const LEFT_OF: Self = Self::scalar(FilterSqlOperator(
        str_constants::pg_crud::LEFT_OF_SQL_OPERATOR,
    ));
    pub(super) const OVERLAPS: Self = Self::scalar(FilterSqlOperator(
        str_constants::pg_crud::OVERLAPS_SQL_OPERATOR,
    ));
    pub(super) const RIGHT_OF: Self = Self::scalar(FilterSqlOperator(
        str_constants::pg_crud::RIGHT_OF_SQL_OPERATOR,
    ));
    pub(super) const TEXT_SEARCH: Self = Self {
        bind_count: BindCount(1usize),
        sql_operator: FilterSqlOperator(str_constants::pg_crud::TEXT_SEARCH_SQL_OPERATOR),
        sql_suffix: FilterSqlSuffix(str_constants::pg_crud::TEXT_SEARCH_SQL_SUFFIX),
        value_shape: FilterValueShape::Text,
    };
    pub(super) const WITHIN: Self = Self::scalar(FilterSqlOperator(
        str_constants::pg_crud::WITHIN_SQL_OPERATOR,
    ));
    pub(super) const fn bind_count_matches(
        self,
        value: crate::bind::FilterPlaceholderCount,
    ) -> FilterSpecValid {
        FilterSpecValid(self.bind_count.0 == value.0)
    }
    pub(super) const fn has_text_value_shape(self) -> FilterSpecValid {
        FilterSpecValid(matches!(self.value_shape, FilterValueShape::Text))
    }
    const fn scalar(sql_operator: FilterSqlOperator) -> Self {
        Self {
            bind_count: BindCount(1usize),
            sql_operator,
            sql_suffix: FilterSqlSuffix(str_constants::pg_crud::EMPTY_SQL_SUFFIX),
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
