#[must_use]
pub fn gen_impl_to_err_string_ts<
    ImplGenericsTokenStream,
    IdentifierTokenStream,
    IdentifierGenericsTokenStream,
    BodyTokenStream,
>(
    impl_generics: &ImplGenericsTokenStream,
    identifier: &IdentifierTokenStream,
    identifier_generics: &IdentifierGenericsTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    ImplGenericsTokenStream: quote::ToTokens + ?Sized,
    IdentifierTokenStream: quote::ToTokens + ?Sized,
    IdentifierGenericsTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        impl #impl_generics to_err_string::ToErrString for #identifier #identifier_generics {
            fn to_err_string(&self) -> to_err_string::ErrorString {
                #body
            }
        }
    }
}
