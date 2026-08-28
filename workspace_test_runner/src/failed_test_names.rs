use crate::execution::{CommandText, CommandTexts, TextRef};

#[allow(clippy::single_call_fn)] // named command or composition stage has one orchestration owner
pub(super) fn failed_test_names(log_text: TextRef<'_>) -> CommandTexts {
    let mut names = log_text
        .get()
        .lines()
        .filter_map(|line| {
            line.strip_prefix(constants_str::TEST_ALT)
                .and_then(|tail| tail.strip_suffix(constants_str::FAILED_ALT))
                .or_else(|| {
                    let tail = line.strip_prefix(constants_str::FOUR_SPACES)?;
                    tail.strip_suffix(constants_str::FAILED)
                })
                .map(|name| {
                    CommandText::try_from(name.to_owned()).unwrap_or_else(CommandText::from)
                })
        })
        .collect::<Vec<CommandText>>();
    names.sort_by(|left, right| left.as_ref().cmp(right.as_ref()));
    names.dedup_by(|left, right| left.as_ref() == right.as_ref());
    CommandTexts::from(bounded_types::domain_types::vector::BoundedVec::from_max_iter(names))
}
