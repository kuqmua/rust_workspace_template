pub(crate) fn naming_title_case(
    value: crate::project_name_ref::ProjectNameRef<'_>,
) -> crate::scaffold_text::ScaffoldText {
    crate::naming_capitalized_parts::naming_capitalized_parts(
        value,
        crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::SPACE),
    )
}
