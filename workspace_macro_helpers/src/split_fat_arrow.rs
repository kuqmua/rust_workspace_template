use crate::domain_types::ProcMacro2MacroTokens;

#[must_use]
pub fn split_fat_arrow<T>(input: T) -> Option<(ProcMacro2MacroTokens, ProcMacro2MacroTokens)>
where
    T: Into<ProcMacro2MacroTokens>,
{
    let mut before = proc_macro2::TokenStream::new();
    let mut after = proc_macro2::TokenStream::new();
    let mut iter = input.into().0.into_iter().peekable();
    while let Some(token) = iter.next() {
        if let proc_macro2::TokenTree::Punct(punct) = &token
            && punct.as_char() == '='
            && let Some(proc_macro2::TokenTree::Punct(next_punct)) = iter.peek()
            && next_punct.as_char() == '>'
        {
            let arrow = {
                let mut ts = proc_macro2::TokenStream::new();
                ts.extend([token]);
                ts.extend(iter.next());
                ts
            };
            let _: syn::Token![=>] = syn::parse2(arrow).ok()?;
            after.extend(iter);
            return Some((
                ProcMacro2MacroTokens::from(before),
                ProcMacro2MacroTokens::from(after),
            ));
        }
        before.extend([token]);
    }
    None
}
