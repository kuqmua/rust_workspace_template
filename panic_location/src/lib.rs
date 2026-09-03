pub mod panic_column;
pub mod panic_file;
pub mod panic_line;
pub mod panic_with_location_message;

static PANIC_HOOK_ONCE: std::sync::Once = std::sync::Once::new();

pub fn panic_location() {
    PANIC_HOOK_ONCE.call_once(|| {
        std::panic::set_hook(Box::new(move |panic_info| {
            if let Some(location) = panic_info.location() {
                tracing::error!(
                    message = %panic_with_location_message::panic_with_location_message(
                        panic_file::PanicFile::from(location.file()),
                        panic_line::PanicLine::from(location.line()),
                        panic_column::PanicColumn::from(location.column()),
                    ),
                    captured = %constants_str::TRACING_PANIC_CAPTURED
                );
            } else {
                tracing::error!(
                    message = constants_str::PANIC_LOCATION_NO_LOCATION_MSG,
                    captured = %constants_str::TRACING_PANIC_CAPTURED_WITHOUT_LOCATION
                );
            }
        }));
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_panic_location_can_be_called_multiple_times() {
        crate::panic_location();
        crate::panic_location();
    }

    #[test]
    fn test_panic_no_location_message_is_stable() {
        assert_eq!(
            constants_str::PANIC_LOCATION_NO_LOCATION_MSG,
            std::str::from_utf8(b"panic occurred but can't get location information...")
                .expect(constants_str::VALUE_0EF05B85)
        );
    }
}
