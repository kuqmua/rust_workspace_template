pub(super) fn naming_capitalized_parts(
    value: crate::project_name_ref::ProjectNameRef<'_>,
    separator: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
) -> crate::scaffold_text::ScaffoldText {
    let output = value
        .0
        .split('_')
        .filter(|part| !part.is_empty())
        .enumerate()
        .fold(
            String::with_capacity(value.0.len()),
            |mut output, (index, part)| {
                if index > constants_usize::ZERO {
                    output.push_str(separator.0);
                }
                let mut chars = part.chars();
                if let Some(first) = chars.next() {
                    output.extend(first.to_uppercase());
                    output.extend(chars);
                }
                output
            },
        );
    crate::scaffold_text::ScaffoldText::try_from(output)
        .unwrap_or_else(crate::scaffold_text::ScaffoldText::from)
}
