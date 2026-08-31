pub(crate) fn memusage_table_value(
    text: &crate::clean_ansi_text::CleanAnsiText,
    row_name: crate::memusage_row_name::MemusageRowName,
    column_idx: crate::memusage_column_idx::MemusageColumnIdx,
) -> crate::memusage_value_ref::MemusageValueRef<'_> {
    text.as_ref()
        .lines()
        .find(|line| line.contains(row_name.get()))
        .and_then(|line| line.split('|').nth(1))
        .and_then(|tail| tail.split_whitespace().nth(column_idx.get()))
        .map_or_else(
            || crate::memusage_value_ref::MemusageValueRef::from(constants_str::UNAVAILABLE),
            crate::memusage_value_ref::MemusageValueRef::from,
        )
}
