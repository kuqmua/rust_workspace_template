static PANIC_HOOK_ONCE: std::sync::Once = std::sync::Once::new();

#[derive(Debug, Clone, Copy)]
pub struct PanicLocationFile<'file>(&'file str);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PanicLocationLine(String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PanicLocationColumn(String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PanicLocationMessage(String);

impl<'file> From<&'file str> for PanicLocationFile<'file> {
    fn from(value: &'file str) -> Self {
        Self(value)
    }
}

impl From<u32> for PanicLocationLine {
    fn from(value: u32) -> Self {
        Self(value.to_string())
    }
}

impl From<u32> for PanicLocationColumn {
    fn from(value: u32) -> Self {
        Self(value.to_string())
    }
}

impl AsRef<str> for PanicLocationMessage {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[must_use]
pub fn panic_with_location_message(
    file: PanicLocationFile<'_>,
    line: &PanicLocationLine,
    column: &PanicLocationColumn,
) -> PanicLocationMessage {
    PanicLocationMessage(format!(
        "{}{}:{}:{}",
        naming_constants::MESSAGE_PANIC_OCCURRED_IN_LOCATION_PREFIX,
        file.0,
        line.0,
        column.0
    ))
}

pub fn panic_loc() {
    PANIC_HOOK_ONCE.call_once(|| {
        std::panic::set_hook(Box::new(move |panic_info| {
            let mut standard_error = std::io::stderr();
            if let Some(location) = panic_info.location() {
                let panic_location_line = PanicLocationLine::from(location.line());
                let panic_location_column = PanicLocationColumn::from(location.column());
                let panic_location_message = panic_with_location_message(
                    PanicLocationFile(location.file()),
                    &panic_location_line,
                    &panic_location_column,
                );
                if std::io::Write::write_fmt(
                    &mut standard_error,
                    format_args!("{}\n", panic_location_message.as_ref()),
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

    #[test]
    fn panic_with_location_message_is_formatted_as_expected() -> Result<(), String> {
        let panic_location_line = crate::PanicLocationLine::from(7);
        let panic_location_column = crate::PanicLocationColumn::from(11);
        let actual_message = crate::panic_with_location_message(
            crate::PanicLocationFile("src/lib.rs"),
            &panic_location_line,
            &panic_location_column,
        );
        let expected_message = "panic occurred in src/lib.rs:7:11";
        if actual_message.as_ref() == expected_message {
            return Ok(());
        }
        Err(format!("{} != {expected_message}", actual_message.as_ref()))
    }
}
