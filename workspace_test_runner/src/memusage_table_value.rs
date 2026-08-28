use super::{CleanAnsiText, MemusageColumnIdx, MemusageRowName, MemusageValueRef};

pub(crate) fn memusage_table_value(
    text: &CleanAnsiText,
    row_name: MemusageRowName,
    column_idx: MemusageColumnIdx,
) -> MemusageValueRef<'_> {
    text.0
        .as_str()
        .lines()
        .find(|line| line.contains(row_name.get()))
        .and_then(|line| line.split('|').nth(1))
        .and_then(|tail| tail.split_whitespace().nth(column_idx.get()))
        .map_or_else(
            || MemusageValueRef::from(constants_str::UNAVAILABLE),
            MemusageValueRef,
        )
}
