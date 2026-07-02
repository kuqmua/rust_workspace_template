static PANIC_HOOK_ONCE: std::sync::Once = std::sync::Once::new();

const MESSAGE_PANIC_WITHOUT_LOCATION: &str = "panic occurred but can't get location information...";

pub fn panic_loc() {
    PANIC_HOOK_ONCE.call_once(|| {
        std::panic::set_hook(Box::new(move |panic_info| {
            let mut standard_error = std::io::stderr();
            if let Some(location) = panic_info.location() {
                if std::io::Write::write_fmt(
                    &mut standard_error,
                    format_args!(
                        "panic occurred in {}:{}:{}\n",
                        location.file(),
                        location.line(),
                        location.column()
                    ),
                )
                .is_err()
                {
                    return;
                }
                return;
            }
            let _write_result = std::io::Write::write_fmt(
                &mut standard_error,
                format_args!("{MESSAGE_PANIC_WITHOUT_LOCATION}\n"),
            );
        }));
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn panic_loc_can_be_called_multiple_times() {
        crate::panic_loc();
        crate::panic_loc();
    }

    #[test]
    fn panic_without_location_message_is_stable() -> Result<(), String> {
        if crate::MESSAGE_PANIC_WITHOUT_LOCATION
            == "panic occurred but can't get location information..."
        {
            return Ok(());
        }
        Err("unexpected panic without location message".to_owned())
    }
}
