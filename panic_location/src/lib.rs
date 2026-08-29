pub mod panic_column;
pub mod panic_file;
pub mod panic_line;
pub mod panic_with_location_message;

// Intentional process-wide state: std exposes the panic hook as one global slot, and this guard
// prevents repeatedly replacing that hook from proc-macro entrypoints.
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
                    "panic captured"
                );
            } else {
                tracing::error!(
                    message = constants_str::catalog::PANIC_LOCATION_NO_LOCATION_MSG,
                    "panic captured without a source location"
                );
            }
        }));
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn panic_location_can_be_called_multiple_times() {
        crate::panic_location();
        crate::panic_location();
    }

    #[test]
    fn panic_no_location_message_is_stable() {
        assert_eq!(
            constants_str::catalog::PANIC_LOCATION_NO_LOCATION_MSG,
            "panic occurred but can't get location information..."
        );
    }
}
