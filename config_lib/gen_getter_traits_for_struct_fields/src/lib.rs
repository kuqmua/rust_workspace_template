struct GetterDeriveInput(syn::DeriveInput);

struct GetterFieldType(syn::Type);

fn compile_error_token_stream<MessageText>(message: &MessageText) -> proc_macro::TokenStream
where
    MessageText: AsRef<str> + ?Sized,
{
    syn::Error::new(proc_macro2::Span::call_site(), message.as_ref())
        .to_compile_error()
        .into()
}

#[proc_macro_derive(GenGetterTraitsForStructFields)]
pub fn gen_getter_traits_for_struct_fields(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let GetterDeriveInput(derive_input) = match syn::parse::<syn::DeriveInput>(input_token_stream) {
        Ok(derive_input) => GetterDeriveInput(derive_input),
        Err(error) => return error.to_compile_error().into(),
    };
    let syn::Data::Struct(data_struct) = derive_input.data else {
        return compile_error_token_stream(
            naming_constants::MESSAGE_GEN_GETTER_TRAITS_FOR_STRUCT_FIELDS_SUPPORTS_ONLY_STRUCTS,
        );
    };
    let struct_identifier = derive_input.ident;
    let generated = data_struct.fields.into_iter().map(|field| {
        let Some(field_identifier) = field.ident else {
            return proc_macro2::TokenStream::from(compile_error_token_stream(
                naming_constants::MESSAGE_GEN_GETTER_TRAITS_FOR_STRUCT_FIELDS_SUPPORTS_ONLY_NAMED_FIELDS,
            ));
        };
        let GetterFieldType(field_type) = GetterFieldType(field.ty);
        let converted_case_text = naming_cmn::ToTokensToUccStr::case(&field_identifier);
        let trait_identifier_text = format!("app_state::Get{}", converted_case_text.as_ref());
        let trait_identifier = match trait_identifier_text.parse::<proc_macro2::TokenStream>() {
            Ok(trait_identifier) => trait_identifier,
            Err(error) => {
                return proc_macro2::TokenStream::from(compile_error_token_stream(
                    &error.to_string(),
                ));
            }
        };
        let method_identifier_text = format!("get_{field_identifier}");
        let method_identifier = match method_identifier_text.parse::<proc_macro2::TokenStream>() {
            Ok(method_identifier) => method_identifier,
            Err(error) => {
                return proc_macro2::TokenStream::from(compile_error_token_stream(
                    &error.to_string(),
                ));
            }
        };
        quote::quote! {
            impl #trait_identifier for #struct_identifier {
                fn #method_identifier(&self) -> &#field_type {
                    &self.#field_identifier
                }
            }

            impl #trait_identifier for &#struct_identifier {
                fn #method_identifier(&self) -> &#field_type {
                    &self.#field_identifier
                }
            }
        }
    });
    quote::quote! { #(#generated)* }.into()
}

#[proc_macro_derive(GenGetterTrait)]
pub fn gen_getter_trait(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let GetterDeriveInput(derive_input) = match syn::parse::<syn::DeriveInput>(input_token_stream) {
        Ok(derive_input) => GetterDeriveInput(derive_input),
        Err(error) => return error.to_compile_error().into(),
    };
    let syn::Data::Struct(data_struct) = derive_input.data else {
        return compile_error_token_stream(
            naming_constants::MESSAGE_GEN_GETTER_TRAIT_SUPPORTS_ONLY_TUPLE_STRUCTS,
        );
    };
    let syn::Fields::Unnamed(fields_unnamed) = data_struct.fields else {
        return compile_error_token_stream(
            naming_constants::MESSAGE_GEN_GETTER_TRAIT_SUPPORTS_ONLY_TUPLE_STRUCTS,
        );
    };
    let Some(first_field) = fields_unnamed.unnamed.first() else {
        return compile_error_token_stream(
            naming_constants::MESSAGE_GEN_GETTER_TRAIT_REQUIRES_ONE_TUPLE_FIELD,
        );
    };
    if fields_unnamed.unnamed.len() != 1 {
        return compile_error_token_stream(
            naming_constants::MESSAGE_GEN_GETTER_TRAIT_REQUIRES_ONE_TUPLE_FIELD,
        );
    }
    let identifier = derive_input.ident;
    let field_type = &first_field.ty;
    let get_identifier_upper_camel = naming_cmn::ToTokensToUccStr::case(&identifier);
    let get_identifier_snake = naming_cmn::ToTokensToScStr::case(&identifier);
    let trait_identifier_text = format!("Get{}", get_identifier_upper_camel.as_ref());
    let method_identifier_text = format!("get_{}", get_identifier_snake.as_ref());
    let trait_identifier = match trait_identifier_text.parse::<proc_macro2::TokenStream>() {
        Ok(trait_identifier) => trait_identifier,
        Err(error) => return compile_error_token_stream(&error.to_string()),
    };
    let method_identifier = match method_identifier_text.parse::<proc_macro2::TokenStream>() {
        Ok(method_identifier) => method_identifier,
        Err(error) => return compile_error_token_stream(&error.to_string()),
    };
    quote::quote! {
        pub trait #trait_identifier {
            fn #method_identifier(&self) -> &#field_type;
        }
    }
    .into()
}
