struct TypeListInput {
    types: syn::punctuated::Punctuated<syn::Type, syn::Token![,]>,
}

struct TypeMessageInput {
    pairs: syn::punctuated::Punctuated<TypeMessagePair, syn::Token![,]>,
}

struct TypeMessagePair {
    message: syn::LitStr,
    ty: syn::Type,
}

impl syn::parse::Parse for TypeListInput {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            types: syn::punctuated::Punctuated::parse_terminated(input)?,
        })
    }
}

impl syn::parse::Parse for TypeMessageInput {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            pairs: syn::punctuated::Punctuated::parse_terminated(input)?,
        })
    }
}

impl syn::parse::Parse for TypeMessagePair {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let ty = input.parse::<syn::Type>()?;
        let _fat_arrow_token = input.parse::<syn::Token![=>]>()?;
        let message = input.parse::<syn::LitStr>()?;
        Ok(Self { message, ty })
    }
}

#[proc_macro]
pub fn impl_to_err_string_with_to_string(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TypeListInput = match syn::parse(input_token_stream) {
        Ok(input) => input,
        Err(error) => return error.to_compile_error().into(),
    };
    let implementations = input.types.into_iter().map(|ty| {
        quote::quote! {
            impl crate::ToErrString for #ty {
                fn to_err_string(&self) -> crate::ErrorString {
                    self.to_string().into()
                }
            }
        }
    });
    quote::quote! {
        #(#implementations)*
    }
    .into()
}

#[proc_macro]
pub fn impl_to_err_string_with_as_ref_str(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TypeListInput = match syn::parse(input_token_stream) {
        Ok(input) => input,
        Err(error) => return error.to_compile_error().into(),
    };
    let implementations = input.types.into_iter().map(|ty| {
        quote::quote! {
            impl crate::ToErrString for #ty {
                fn to_err_string(&self) -> crate::ErrorString {
                    AsRef::<str>::as_ref(self).to_owned().into()
                }
            }
        }
    });
    quote::quote! {
        #(#implementations)*
    }
    .into()
}

#[proc_macro]
pub fn impl_to_err_string_with_static_message(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TypeMessageInput = match syn::parse(input_token_stream) {
        Ok(input) => input,
        Err(error) => return error.to_compile_error().into(),
    };
    let implementations = input.pairs.into_iter().map(|pair| {
        let ty = pair.ty;
        let message = pair.message;
        quote::quote! {
            impl crate::ToErrString for #ty {
                fn to_err_string(&self) -> crate::ErrorString {
                    #message.to_owned().into()
                }
            }
        }
    });
    quote::quote! {
        #(#implementations)*
    }
    .into()
}
