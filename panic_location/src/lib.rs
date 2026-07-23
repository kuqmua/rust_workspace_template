// Intentional process-wide state: std exposes the panic hook as one global slot, and this guard
// prevents repeatedly replacing that hook from proc-macro entrypoints.
static PANIC_HOOK_ONCE: std::sync::Once = std::sync::Once::new();
#[derive(Clone, Copy, newtype::FromInner)]
struct PanicFile<'file_lt>(&'file_lt str);
#[derive(Clone, Copy, newtype::FromInner)]
struct PanicLine(u32);
#[derive(Clone, Copy, newtype::FromInner)]
struct PanicColumn(u32);
#[allow(clippy::single_call_fn)] // keeps panic message construction reusable and testable in one place
fn panic_with_location_message(
    file: PanicFile<'_>,
    line: PanicLine,
    column: PanicColumn,
) -> to_err_string::ErrorText {
    to_err_string::ErrorText::try_from(format!(
        "panic occurred in {}:{}:{}",
        file.0, line.0, column.0
    ))
    .unwrap_or_else(to_err_string::ErrorText::from)
}
pub fn panic_location() {
    PANIC_HOOK_ONCE.call_once(|| {
        std::panic::set_hook(Box::new(move |panic_info| {
            if let Some(location) = panic_info.location() {
                eprintln!(
                    "{}",
                    panic_with_location_message(
                        PanicFile(location.file()),
                        PanicLine(location.line()),
                        PanicColumn(location.column())
                    )
                    .as_ref()
                );
            } else {
                eprintln!("{}", str_constants::PANIC_LOCATION_NO_LOCATION_MSG);
            }
        }));
    });
}
#[cfg(test)]
mod tests {
    #[test]
    fn panic_location_can_be_called_multiple_times() {
        super::panic_location();
        super::panic_location();
    }
    #[test]
    fn panic_no_location_message_is_stable() {
        assert_eq!(
            str_constants::PANIC_LOCATION_NO_LOCATION_MSG,
            "panic occurred but can't get location information..."
        );
    }
    #[test]
    fn panic_with_location_message_is_formatted_as_expected() {
        assert_eq!(
            super::panic_with_location_message(
                super::PanicFile("src/lib.rs"),
                super::PanicLine(7),
                super::PanicColumn(11)
            )
            .as_ref(),
            "panic occurred in src/lib.rs:7:11"
        );
    }
}
