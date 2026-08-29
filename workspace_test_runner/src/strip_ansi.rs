use super::execution::{CommandText, TextRef};

#[allow(clippy::single_call_fn)] // named command or composition stage has one orchestration owner
pub(super) fn strip_ansi(value: TextRef<'_>) -> CommandText {
    let output = value
        .get()
        .chars()
        .fold(
            (String::with_capacity(value.get().len()), false),
            |(mut output, escaping), character| match (escaping, character) {
                (true, 'm') => (output, false),
                (true, _) | (false, '\u{1b}') => (output, true),
                (false, _) => {
                    output.push(character);
                    (output, false)
                }
            },
        )
        .0;
    CommandText::try_from(output).unwrap_or_else(CommandText::from)
}
