struct TryFromEnvInput(syn::DeriveInput);

fn compile_error_token_stream<MessageText>(message: &MessageText) -> proc_macro::TokenStream
where
    MessageText: AsRef<str> + ?Sized,
{
    syn::Error::new(proc_macro::Span::call_site().into(), message.as_ref())
        .to_compile_error()
        .into()
}

#[proc_macro_derive(TryFromEnv)]
pub fn try_from_env(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let TryFromEnvInput(derive_input) = match syn::parse::<syn::DeriveInput>(input_token_stream) {
        Ok(derive_input) => TryFromEnvInput(derive_input),
        Err(error) => return error.to_compile_error().into(),
    };
    let syn::Data::Struct(data_struct) = derive_input.data else {
        return compile_error_token_stream("TryFromEnv supports only structs");
    };
    let syn::Fields::Named(fields_named) = data_struct.fields else {
        return compile_error_token_stream("TryFromEnv supports only named fields");
    };
    let struct_identifier = derive_input.ident;
    let field_initializers = fields_named.named.into_iter().map(|field| {
        let Some(field_identifier) = field.ident else {
            return quote::quote! {};
        };
        let field_type = field.ty;
        quote::quote! {
            #field_identifier: <#field_type as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                config_lib::EnvVarName::from_identifier_text(stringify!(#field_identifier)),
            ),
        }
    });
    quote::quote! {
        impl #struct_identifier {
            pub fn try_from_env() -> Result<Self, config_lib::TryFromEnvError> {
                Ok(Self {
                    #(#field_initializers)*
                })
            }
        }
    }
    .into()
}
