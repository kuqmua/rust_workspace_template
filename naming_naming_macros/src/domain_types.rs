#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    newtype::ToTokens,
)]
pub(crate) struct ProcMacro2GeneratedNamingTokenStream(proc_macro2::TokenStream);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::AsRefInner, newtype::FromInner,
)]
pub(crate) struct SynEnumIdentifierRef<'identifier_lt>(&'identifier_lt syn::Ident);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub(crate) struct ProcMacro2VariantMatchingTokensRef<'tokens_lt>(
    &'tokens_lt [proc_macro2::TokenStream],
);
