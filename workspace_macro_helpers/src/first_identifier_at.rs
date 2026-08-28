use crate::domain_types::{
    FirstIdentifier, PartIndex, ProcMacro2TopLevelCommaParts, parse_first_identifier, part_at,
};

#[must_use]
pub fn first_identifier_at<I>(
    parts: &ProcMacro2TopLevelCommaParts,
    idx: I,
) -> Option<FirstIdentifier>
where
    I: Into<PartIndex>,
{
    parse_first_identifier(&mut part_at(parts, idx)?.into_iter())
}
