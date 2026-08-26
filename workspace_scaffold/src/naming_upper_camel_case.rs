#[allow(
    clippy::single_call_fn,
    reason = "service scaffold owns identifier case conversion"
)]
pub(super) fn upper_camel_case(value: super::ProjectNameRef<'_>) -> super::ScaffoldText {
    super::naming_capitalized_parts::capitalized_parts(
        value,
        super::ScaffoldTextRef::from(constants_str::EMPTY),
    )
}
