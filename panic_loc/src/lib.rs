// Intentional process-wide state: std exposes the panic hook as one global slot, and this guard
// prevents repeatedly replacing that hook from proc-macro entrypoints.
static PANIC_HOOK_ONCE: std::sync::Once = std::sync::Once::new();
const PANIC_NO_LOCATION_MSG: &str = "panic occurred but can't get location information...";
const PANIC_WITH_LOCATION_MSG_MAX_LEN: usize = 1_048_576;
#[derive(Clone, Copy)]
struct PanicFile<'file_lt>(&'file_lt str);
#[derive(Clone, Copy)]
struct PanicLine(u32);
#[derive(Clone, Copy)]
struct PanicCol(u32);
#[derive(newtype::BoundedString)]
#[bounded_string(
    max = PANIC_WITH_LOCATION_MSG_MAX_LEN,
    description = "panic location message"
)]
struct PanicWithLocationMsg(String);
#[allow(clippy::single_call_fn)] // keeps panic message construction reusable and testable in one place
fn panic_with_location_msg(
    file: PanicFile<'_>,
    line: PanicLine,
    col: PanicCol,
) -> PanicWithLocationMsg {
    PanicWithLocationMsg::try_from(format!("panic occurred in {}:{}:{}", file.0, line.0, col.0))
        .unwrap_or_else(PanicWithLocationMsg::from)
}
pub fn panic_loc() {
    PANIC_HOOK_ONCE.call_once(|| {
        std::panic::set_hook(Box::new(move |panic_info| {
            if let Some(location) = panic_info.location() {
                eprintln!(
                    "{}",
                    panic_with_location_msg(
                        PanicFile(location.file()),
                        PanicLine(location.line()),
                        PanicCol(location.column())
                    )
                    .0
                );
            } else {
                eprintln!("{PANIC_NO_LOCATION_MSG}");
            }
        }));
    });
}
#[cfg(test)]
mod tests {
    #[test]
    fn panic_loc_can_be_called_multiple_times() {
        super::panic_loc();
        super::panic_loc();
    }
    #[test]
    fn panic_no_location_message_is_stable() {
        assert_eq!(
            super::PANIC_NO_LOCATION_MSG,
            "panic occurred but can't get location information..."
        );
    }
    #[test]
    fn panic_with_location_message_is_formatted_as_expected() {
        assert_eq!(
            super::panic_with_location_msg(
                super::PanicFile("src/lib.rs"),
                super::PanicLine(7),
                super::PanicCol(11)
            )
            .0,
            "panic occurred in src/lib.rs:7:11"
        );
    }
}
