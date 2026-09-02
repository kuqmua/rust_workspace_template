pub(crate) fn memusage_table_value(
    clean_ansi_text: &crate::clean_ansi_text::CleanAnsiText,
    memusage_row_name: crate::memusage_row_name::MemusageRowName,
    memory_usage_column_index: crate::memory_usage_column_index::MemoryUsageColumnIndex,
) -> crate::memusage_value_ref::MemusageValueRef<'_> {
    clean_ansi_text
        .as_ref()
        .lines()
        .find(|line| line.contains(memusage_row_name.get()))
        .and_then(|line| line.split('|').nth(1))
        .and_then(|tail| tail.split_whitespace().nth(memory_usage_column_index.get()))
        .map_or_else(
            || crate::memusage_value_ref::MemusageValueRef::from(constants_str::UNAVAILABLE),
            crate::memusage_value_ref::MemusageValueRef::from,
        )
}
