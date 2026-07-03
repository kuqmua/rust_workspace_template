#[must_use]
pub fn gen_if_write_is_err_ts<ParametersTokenStream, BodyTokenStream>(
    parameters: &ParametersTokenStream,
    body: &BodyTokenStream,
) -> proc_macro2::TokenStream
where
    ParametersTokenStream: quote::ToTokens + ?Sized,
    BodyTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        if write!(#parameters, #body).is_err() {
            return std::fmt::Result::Err(std::fmt::Error);
        }
    }
}
