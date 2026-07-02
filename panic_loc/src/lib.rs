static PANIC_HOOK_ONCE: std::sync::Once = std::sync::Once::new();

pub fn panic_loc() {
    PANIC_HOOK_ONCE.call_once(|| {
        std::panic::set_hook(Box::new(move |panic_info| {
            let mut standard_error = std::io::stderr();
            if let Some(location) = panic_info.location() {
                if std::io::Write::write_fmt(
                    &mut standard_error,
                    format_args!(
                        "{}{}:{}:{}\n",
                        naming_constants::MESSAGE_PANIC_OCCURRED_IN_LOCATION_PREFIX,
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
                format_args!("{}\n", naming_constants::MESSAGE_PANIC_WITHOUT_LOCATION),
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
        if !naming_constants::MESSAGE_PANIC_WITHOUT_LOCATION.is_empty() {
            return Ok(());
        }
        Err(naming_constants::MESSAGE_PANIC_WITHOUT_LOCATION_TEST_UNEXPECTED.to_owned())
    }
}
