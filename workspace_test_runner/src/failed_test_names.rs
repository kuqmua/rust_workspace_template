#[allow(clippy::single_call_fn)] // named command or composition stage has one orchestration owner
pub(super) fn failed_test_names(
    log_text: crate::text_ref::TextRef<'_>,
) -> crate::command_texts::CommandTexts {
    let mut names = log_text
        .as_ref()
        .lines()
        .filter_map(|line| {
            line.strip_prefix(constants_str::catalog::TEST_ALT)
                .and_then(|tail| tail.strip_suffix(constants_str::catalog::FAILED_ALT))
                .or_else(|| {
                    let tail = line.strip_prefix(constants_str::catalog::FOUR_SPACES)?;
                    tail.strip_suffix(constants_str::catalog::FAILED)
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
