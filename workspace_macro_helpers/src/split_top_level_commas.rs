pub fn split_top_level_commas<T>(
    t: T,
) -> crate::proc_macro2_top_level_comma_parts::ProcMacro2TopLevelCommaParts
where
    T: Into<crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens>,
{
    syn::parse2::<crate::proc_macro2_top_level_comma_parts::ProcMacro2TopLevelCommaParts>(
        t.into().into_inner(),
    )
    .unwrap_or_default()
}
