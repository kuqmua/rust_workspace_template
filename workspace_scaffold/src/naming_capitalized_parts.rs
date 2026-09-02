pub(super) fn naming_capitalized_parts(
    project_name_ref: crate::project_name_ref::ProjectNameRef<'_>,
    scaffold_text_ref: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
) -> crate::scaffold_text::ScaffoldText {
    let output = project_name_ref
        .get()
        .split('_')
        .filter(|part| !part.is_empty())
        .enumerate()
        .fold(
            String::with_capacity(project_name_ref.get().len()),
            |mut output, (index, part)| {
                if index > constants_usize::ZERO {
                    output.push_str(scaffold_text_ref.get());
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
