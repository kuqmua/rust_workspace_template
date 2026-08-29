#[must_use]
pub fn first_identifier_at<I>(
    parts: &crate::proc_macro2_top_level_comma_parts::ProcMacro2TopLevelCommaParts,
    idx: I,
) -> Option<crate::first_identifier::FirstIdentifier>
where
    I: Into<crate::part_index::PartIndex>,
{
    crate::parse_first_identifier::parse_first_identifier(
        &mut crate::part_at::part_at(parts, idx)?.into_iter(),
    )
}
