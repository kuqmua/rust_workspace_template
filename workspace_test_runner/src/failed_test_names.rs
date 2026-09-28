#[allow(
    clippy::single_call_fn,
    reason = "failed test names remains a named owner because its boundary role is clearer and directly testable"
)]
pub(super) fn failed_test_names(
    text_ref: crate::text_ref::TextRef<'_>,
) -> crate::command_texts::CommandTexts {
    let clean_log =
        crate::strip_ansi::strip_ansi(macro_helpers::tool_ansi_chars::ToolAnsiChars::from(
            macro_helpers::tool_ansi_text_ref::ToolAnsiTextRef::from(text_ref.as_ref()),
        ));
    let mut names = clean_log
        .as_ref()
        .lines()
        .filter_map(|line| {
            line.strip_prefix(constants_str::TEST_ALT)
                .and_then(|tail| tail.strip_suffix(constants_str::FAILED_ALT))
                .or_else(|| {
                    let tail = line.strip_prefix(constants_str::FOUR_SPACES)?;
                    tail.strip_suffix(constants_str::FAILED)
                })
                .map(|name| {
                    crate::command_text::CommandText::try_from(name.to_owned())
                        .unwrap_or_else(crate::command_text::CommandText::from)
                })
        })
        .collect::<Vec<crate::command_text::CommandText>>();
    names.sort_by(|left, right| left.as_ref().cmp(right.as_ref()));
    names.dedup_by(|left, right| left.as_ref() == right.as_ref());
    crate::command_texts::CommandTexts::from(bounded_types::bounded_vec::BoundedVec::from_max_iter(
        names,
    ))
}
