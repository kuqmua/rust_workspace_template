#[allow(
    clippy::single_call_fn,
    reason = "service scaffold owns title case conversion"
)]
pub(crate) fn title_case(value: super::ProjectNameRef<'_>) -> super::ScaffoldText {
    super::naming_capitalized_parts::capitalized_parts(
        value,
        super::ScaffoldTextRef::from(constants_str::SPACE),
    )
}
