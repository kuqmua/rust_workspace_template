#[allow(clippy::single_call_fn)] // named command or composition stage has one orchestration owner
pub(super) fn strip_ansi(value: crate::text_ref::TextRef<'_>) -> crate::command_text::CommandText {
    let output = value
        .as_ref()
        .chars()
        .fold(
            (String::with_capacity(value.as_ref().len()), false),
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
    crate::command_text::CommandText::try_from(output)
        .unwrap_or_else(crate::command_text::CommandText::from)
}
