#[must_use]
pub fn gen_impl_from_ts<FromTokenStream, ForTokenStream, BodyTokenStream>(
    from_type: &FromTokenStream,
    for_type: &ForTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    FromTokenStream: quote::ToTokens + ?Sized,
    ForTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl From<#from_type> for #for_type {
            fn from(value: #from_type) -> Self {
                #body
            }
        }
    }
}
