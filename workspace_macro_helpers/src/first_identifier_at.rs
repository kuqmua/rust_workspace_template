#[must_use]
pub fn first_identifier_at<I>(
    proc_macro2_top_level_comma_parts: &crate::proc_macro2_top_level_comma_parts::ProcMacro2TopLevelCommaParts,
    i: I,
) -> Option<crate::first_identifier::FirstIdentifier>
where
    I: Into<crate::part_index::PartIndex>,
{
    crate::parse_first_identifier::parse_first_identifier(
        &mut crate::part_at::part_at(proc_macro2_top_level_comma_parts, i)?.into_iter(),
    )
}
