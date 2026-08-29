#[must_use]
pub fn closure_identifier_and_body<T>(
    input: T,
) -> Option<(
    crate::first_identifier::FirstIdentifier,
    crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens,
)>
where
    T: Into<crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens>,
{
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    struct ClosureIdentifierAndBody {
        body: crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens,
        identifier: syn::Ident,
    }
    impl syn::parse::Parse for ClosureIdentifierAndBody {
        fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
            let _: syn::Token![|] = input.parse()?;
            let identifier = input.parse::<syn::Ident>()?;
            let _: syn::Token![|] = input.parse()?;
            let body = input.parse::<crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens>()?;
            Ok(Self { body, identifier })
        }
    }
    let parsed = syn::parse2::<ClosureIdentifierAndBody>(input.into().into_inner()).ok()?;
    Some((
        crate::first_identifier::FirstIdentifier::try_from(parsed.identifier.to_string())
            .unwrap_or_else(crate::first_identifier::FirstIdentifier::from),
        parsed.body,
    ))
}
