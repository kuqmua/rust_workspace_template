use crate::domain_types::{ProcMacro2MacroTokens, ProcMacro2TopLevelCommaParts};

pub fn split_top_level_commas<T>(input: T) -> ProcMacro2TopLevelCommaParts
where
    T: Into<ProcMacro2MacroTokens>,
{
    syn::parse2::<ProcMacro2TopLevelCommaParts>(input.into().into_inner()).unwrap_or_default()
}
