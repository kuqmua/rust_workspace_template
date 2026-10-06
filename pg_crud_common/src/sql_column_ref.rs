#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype_display::Display,
)]
pub struct SqlColumnRef<'column_lt>(&'column_lt dyn std::fmt::Display);
impl<'column_lt, T> From<&'column_lt T> for SqlColumnRef<'column_lt>
where
    T: std::fmt::Display,
{
    fn from(value: &'column_lt T) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SqlColumnRef<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_tuple(constants_str::SQLCOLUMNREF).finish()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_column_debug_avoids_display_and_display_preserves_formatting_failure() {
        #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
        struct SqlColumnFormattingFailure;

        impl std::fmt::Display for SqlColumnFormattingFailure {
            fn fmt(&self, _formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Err(std::fmt::Error)
            }
        }

        let formatting_failure = SqlColumnFormattingFailure;
        let sql_column_ref = crate::sql_column_ref::SqlColumnRef::from(&formatting_failure);
        let copied_column_ref = sql_column_ref;
        assert_eq!(format!("{sql_column_ref:?}"), constants_str::SQLCOLUMNREF);
        assert_eq!(
            format!("{copied_column_ref:#?}"),
            constants_str::SQLCOLUMNREF
        );
        let mut destination = constants_str::X.to_owned();
        assert_eq!(
            std::fmt::Write::write_fmt(&mut destination, format_args!("{sql_column_ref}")),
            Err(std::fmt::Error),
        );
        assert_eq!(destination, constants_str::X);
    }
}
