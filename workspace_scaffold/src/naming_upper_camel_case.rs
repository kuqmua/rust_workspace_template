pub(super) fn naming_upper_camel_case(
    project_name_ref: crate::project_name_ref::ProjectNameRef<'_>,
) -> Result<crate::scaffold_text::ScaffoldText, crate::scaffold_error::ScaffoldError> {
    crate::naming_capitalized_parts::naming_capitalized_parts(
        project_name_ref,
        crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::EMPTY),
    )
}
