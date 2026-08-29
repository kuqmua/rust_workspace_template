#[must_use]
pub fn part_at<I>(
    parts: &crate::proc_macro2_top_level_comma_parts::ProcMacro2TopLevelCommaParts,
    idx: I,
) -> Option<crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens>
where
    I: Into<crate::part_index::PartIndex>,
{
    parts
        .get(idx.into().0)
        .cloned()
        .map(crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from)
}
