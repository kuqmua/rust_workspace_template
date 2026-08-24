#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct PanicFile<'file_lt>(&'file_lt str);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct PanicLine(u32);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct PanicColumn(u32);

#[allow(clippy::single_call_fn)] // keeps panic message construction reusable and testable in one place
pub(crate) fn panic_with_location_message(
    file: PanicFile<'_>,
    line: PanicLine,
    column: PanicColumn,
) -> to_err_string::domain_types::ErrorText {
    to_err_string::domain_types::ErrorText::try_from(format!(
        "panic occurred in {}:{}:{}",
        file.0, line.0, column.0
    ))
    .unwrap_or_else(to_err_string::domain_types::ErrorText::from)
}

#[cfg(test)]
mod tests {
    #[test]
    fn panic_with_location_message_is_formatted_as_expected() {
        assert_eq!(
            super::panic_with_location_message(
                super::PanicFile::from("src/lib.rs"),
                super::PanicLine::from(7),
                super::PanicColumn::from(11),
            )
            .as_ref(),
            "panic occurred in src/lib.rs:7:11"
        );
    }
}
