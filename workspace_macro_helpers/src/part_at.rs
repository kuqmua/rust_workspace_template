#[must_use]
pub fn part_at<I>(
    proc_macro2_top_level_comma_parts: &crate::proc_macro2_top_level_comma_parts::ProcMacro2TopLevelCommaParts,
    i: I,
) -> Option<crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens>
where
    I: Into<crate::part_index::PartIndex>,
{
    proc_macro2_top_level_comma_parts
        .get(i.into().get())
        .cloned()
        .map(crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from)
}
