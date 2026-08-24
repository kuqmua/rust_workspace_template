mod domain_types;

// Intentional process-wide state: std exposes the panic hook as one global slot, and this guard
// prevents repeatedly replacing that hook from proc-macro entrypoints.
static PANIC_HOOK_ONCE: std::sync::Once = std::sync::Once::new();

pub fn panic_location() {
    PANIC_HOOK_ONCE.call_once(|| {
        std::panic::set_hook(Box::new(move |panic_info| {
            if let Some(location) = panic_info.location() {
                tracing::error!(
                    message = %domain_types::panic_with_location_message(
                        domain_types::PanicFile::from(location.file()),
                        domain_types::PanicLine::from(location.line()),
                        domain_types::PanicColumn::from(location.column()),
                    ),
                    "panic captured"
                );
            } else {
                tracing::error!(
                    message = constants_str::PANIC_LOCATION_NO_LOCATION_MSG,
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
        super::panic_location();
        super::panic_location();
    }

    #[test]
    fn panic_no_location_message_is_stable() {
        assert_eq!(
            constants_str::PANIC_LOCATION_NO_LOCATION_MSG,
            "panic occurred but can't get location information..."
        );
    }
}
