pub(crate) fn panic_with_location_message(
    panic_file: crate::panic_file::PanicFile<'_>,
    panic_line: crate::panic_line::PanicLine,
    panic_column: crate::panic_column::PanicColumn,
) -> to_err_string::error_text::ErrorText {
    to_err_string::error_text::ErrorText::try_from(format!(
        "panic occurred in {}:{}:{}",
        panic_file.get(),
        panic_line.get(),
        panic_column.get()
    ))
    .unwrap_or_else(to_err_string::error_text::ErrorText::from)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_panic_with_location_message_is_formatted_as_expected() {
        assert_eq!(
            crate::panic_with_location_message::panic_with_location_message(
                crate::panic_file::PanicFile::from(constants_str::SRC_LIB_RS),
                crate::panic_line::PanicLine::from(7),
                crate::panic_column::PanicColumn::from(11),
            )
            .as_ref(),
            constants_str::VALUE_CCD65EB4
        );
    }
}
