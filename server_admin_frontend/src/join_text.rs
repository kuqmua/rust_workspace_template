pub(crate) fn join_text<'value_lt, Values>(
    values: Values,
) -> crate::domain_types::shared::text::AdminJoinedText
where
    Values: IntoIterator<Item = &'value_lt str>,
{
    let value_iter = values.into_iter();
    let mut text = String::with_capacity(value_iter.size_hint().0.saturating_mul(16usize));
    value_iter.enumerate().for_each(|(index, value)| {
        if index > constants_usize::ZERO {
            text.push_str(constants_str::COMMA_SPACE);
        }
        text.push_str(value);
    });
    crate::domain_types::shared::text::AdminJoinedText::try_from(text)
        .unwrap_or_else(crate::domain_types::shared::text::AdminJoinedText::from)
}

#[cfg(test)]
mod tests {
    #[test]
    fn joins_borrowed_text_without_an_intermediate_collection() {
        assert_eq!(
            super::join_text(["reader", "editor"]).as_ref(),
            "reader, editor"
        );
        assert!(
            super::join_text(std::iter::empty::<&str>())
                .as_ref()
                .is_empty()
        );
    }
}
