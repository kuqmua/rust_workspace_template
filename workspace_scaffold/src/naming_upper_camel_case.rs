#[allow(clippy::single_call_fn)] // names the upper-camel projection used by scaffold replacement construction
pub(super) fn naming_upper_camel_case(value: super::ProjectNameRef<'_>) -> super::ScaffoldText {
    super::naming_capitalized_parts::naming_capitalized_parts(
        value,
        super::ScaffoldTextRef::from(constants_str::EMPTY),
    )
}
