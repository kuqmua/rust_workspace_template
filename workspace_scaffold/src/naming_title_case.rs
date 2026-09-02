pub(crate) fn naming_title_case(
    project_name_ref: crate::project_name_ref::ProjectNameRef<'_>,
) -> crate::scaffold_text::ScaffoldText {
    crate::naming_capitalized_parts::naming_capitalized_parts(
        project_name_ref,
        crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::SPACE),
    )
}
