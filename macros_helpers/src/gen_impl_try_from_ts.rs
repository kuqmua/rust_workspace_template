#[must_use]
pub fn gen_impl_try_from_ts<FromTokenStream, ForTokenStream, ErrorTokenStream, BodyTokenStream>(
    from_type: &FromTokenStream,
    for_type: &ForTokenStream,
    error_type: &ErrorTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    FromTokenStream: quote::ToTokens + ?Sized,
    ForTokenStream: quote::ToTokens + ?Sized,
    ErrorTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl TryFrom<#from_type> for #for_type {
            type Error = #error_type;
            fn try_from(value: #from_type) -> Result<Self, Self::Error> {
                #body
            }
        }
    }
}
