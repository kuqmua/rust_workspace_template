#[must_use]
pub fn gen_impl_dflt_ts<IdentifierTokenStream, BodyTokenStream>(
    identifier: &IdentifierTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl Default for #identifier {
            fn default() -> Self {
                #body
            }
        }
    }
}
