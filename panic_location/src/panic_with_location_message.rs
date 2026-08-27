// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::single_call_fn)] // keeps panic message construction reusable and testable in one place
pub(crate) fn panic_with_location_message(
    file: crate::domain_types::PanicFile<'_>,
    line: crate::domain_types::PanicLine,
    column: crate::domain_types::PanicColumn,
) -> to_err_string::domain_types::ErrorText {
    to_err_string::domain_types::ErrorText::try_from(format!(
        "panic occurred in {}:{}:{}",
        file.get(),
        line.get(),
        column.get()
    ))
    .unwrap_or_else(to_err_string::domain_types::ErrorText::from)
}

#[cfg(test)]
mod tests {
    #[test]
    fn panic_with_location_message_is_formatted_as_expected() {
        assert_eq!(
            super::panic_with_location_message(
                crate::domain_types::PanicFile::from("src/lib.rs"),
                crate::domain_types::PanicLine::from(7),
                crate::domain_types::PanicColumn::from(11),
            )
            .as_ref(),
            "panic occurred in src/lib.rs:7:11"
        );
    }
}
