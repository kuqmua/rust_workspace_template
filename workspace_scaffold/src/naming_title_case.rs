pub(crate) fn naming_title_case(value: super::ProjectNameRef<'_>) -> super::ScaffoldText {
    super::naming_capitalized_parts::naming_capitalized_parts(
        value,
        super::ScaffoldTextRef::from(constants_str::SPACE),
    )
}
