pub(crate) fn authenticated_selected_form_impl<Ids, Parse>(
    auth: crate::AdminAuthReq,
    expected: &crate::AdminHtmlFormText,
    selected: crate::StdAdminHtmlSelected,
    parse: Parse,
) -> Result<(crate::AdminAuthReq, Ids, Ids), crate::AdminError>
where
    Parse: Fn(&crate::AdminHtmlFormText) -> Result<Ids, crate::AdminError>,
{
    let auth = crate::form_auth_impl::form_auth_impl(auth)?;
    let expected = parse(expected)?;
    let separator = constants_str::COMMA_SPACE.trim();
    let selected = bounded_types::domain_types::btree::BoundedBTreeMap::<
        crate::AdminHtmlFormKey,
        crate::AdminHtmlFormText,
        { crate::ADMIN_HTML_FORM_SELECTED_MAX_ITEMS },
    >::from(selected);
    let capacity = selected
        .iter()
        .map(|(_key, value)| value.len().get())
        .sum::<usize>()
        .saturating_add(
            selected
                .len()
                .get()
                .saturating_sub(constants_usize::ONE)
                .saturating_mul(separator.len()),
        );
    let text = selected.into_values().enumerate().fold(
        String::with_capacity(capacity),
        |mut text, (index, value)| {
            if index > constants_usize::ZERO {
                text.push_str(separator);
            }
            text.push_str(value.as_ref());
            text
        },
    );
    let selected_ids = crate::AdminHtmlFormText::try_from(text)
        .map_err(|_error| crate::AdminError::Validation)
        .and_then(|value| parse(&value))?;
    Ok((auth, expected, selected_ids))
}
