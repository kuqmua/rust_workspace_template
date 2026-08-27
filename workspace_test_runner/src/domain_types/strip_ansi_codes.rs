use super::{AnsiTextRef, CleanAnsiText};

pub(crate) fn strip_ansi_codes(value: AnsiTextRef<'_>) -> CleanAnsiText {
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
    CleanAnsiText::try_from(clean).unwrap_or_else(CleanAnsiText::from)
}
