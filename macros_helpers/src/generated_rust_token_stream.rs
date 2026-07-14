#[derive(Debug, Clone, Default, newtype::Newtype)]
#[newtype(
    as_ref_owned,
    deref_inner,
    display,
    from_inner,
    into_inner_from,
    to_tokens
)]
pub struct GeneratedRustTokenStream(proc_macro2::TokenStream);
