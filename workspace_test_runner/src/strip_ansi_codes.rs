pub(crate) fn strip_ansi_codes(
    value: crate::ansi_text_ref::AnsiTextRef<'_>,
) -> crate::clean_ansi_text::CleanAnsiText {
    let clean = value
        .get()
        .chars()
        .fold(
            (String::with_capacity(value.get().len()), false),
            |(mut accumulator, in_escape), ch| match (in_escape, ch) {
                (true, 'm') => (accumulator, false),
                (false, '\u{1b}') | (true, _) => (accumulator, true),
                (false, _) => {
                    accumulator.push(ch);
                    (accumulator, false)
                }
            },
        )
        .0;
    crate::clean_ansi_text::CleanAnsiText::try_from(clean)
        .unwrap_or_else(crate::clean_ansi_text::CleanAnsiText::from)
}
