#[must_use]
pub fn gen_pub_type_al_ts<IdentifierTokenStream, TypeTokenStream>(
    identifier: &IdentifierTokenStream,
    ty: &TypeTokenStream,
) -> proc_macro2::TokenStream
where
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    TypeTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! { pub type #identifier = #ty; }
}
