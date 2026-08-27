use super::{PartIndex, ProcMacro2MacroTokens, ProcMacro2TopLevelCommaParts};

#[must_use]
pub fn part_at<I>(parts: &ProcMacro2TopLevelCommaParts, idx: I) -> Option<ProcMacro2MacroTokens>
where
    I: Into<PartIndex>,
{
    parts
        .get(idx.into().0)
        .cloned()
        .map(ProcMacro2MacroTokens::from)
}
