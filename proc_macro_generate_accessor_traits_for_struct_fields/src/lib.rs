#[proc_macro_derive(GenerateAccessorTraitsForStructFields)]
pub fn generate_accessor_traits_for_struct_fields(
    token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let di: syn::DeriveInput = syn::parse(token_stream).expect(constants_str::DIAGNOSTIC_49780295);
    let identifier = &di.ident;
    let datastruct = match di.data {
        syn::Data::Struct(v) => v,
        syn::Data::Enum(_) | syn::Data::Union(_) => {
            std::panic::panic_any(constants_str::PANIC_15CD72A2)
        }
    };
    let generated_traits_impls_token_stream = datastruct.fields.into_iter().map(|syn_field| {
        let field_type = &syn_field.ty;
        let field_identifier = syn_field
            .ident
            .as_ref()
            .expect(constants_str::DIAGNOSTIC_E5C23C45);
        let upper_camel_case_field =
            naming_common::domain_types::ToTokensToUpperCamelCaseStr::case(&field_identifier);
        let trait_identifier = quote::format_ident!("{}Provider", upper_camel_case_field);
        quote::quote! {
            impl app_state::#trait_identifier for #identifier {
                fn #field_identifier (&self) -> &#field_type {
                    &self.#field_identifier
                }
            }
            impl app_state::#trait_identifier for &#identifier {
                fn #field_identifier (&self) -> &#field_type {
                    &self.#field_identifier
                }
            }
        }
    });
    let generated = quote::quote! {#(#generated_traits_impls_token_stream)*};
    generated.into()
}
#[proc_macro_derive(GenerateAccessorTrait)]
pub fn generate_accessor_trait(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let di: syn::DeriveInput = syn::parse(token_stream).expect(constants_str::DIAGNOSTIC_195B48F5);
    let identifier = &di.ident;
    let data_struct = match di.data {
        syn::Data::Struct(v) => v,
        syn::Data::Enum(_) | syn::Data::Union(_) => {
            std::panic::panic_any(constants_str::PANIC_CD6BBC4E)
        }
    };
    let fields_unnamed = match data_struct.fields {
        syn::Fields::Unnamed(v) => v.unnamed,
        syn::Fields::Named(_) | syn::Fields::Unit => {
            std::panic::panic_any(constants_str::PANIC_577CB86A)
        }
    };
    assert!(fields_unnamed.len() == 1, "1e82dc7e");
    let first_field_unnamed = fields_unnamed
        .iter()
        .next()
        .expect(constants_str::DIAGNOSTIC_7C2531FD);
    let first_field_unnamed_type = &first_field_unnamed.ty;
    let provider_identifier = quote::format_ident!("{}Provider", identifier);
    let accessor_identifier = quote::format_ident!(
        "{}",
        naming_common::domain_types::ToTokensToSnakeCaseStr::case(&identifier)
    );
    let generated = quote::quote! {
        pub trait #provider_identifier {
            fn #accessor_identifier(&self) -> &#first_field_unnamed_type;
        }
        impl<Value> #provider_identifier for &Value
        where
            Value: #provider_identifier + ?Sized,
        {
            fn #accessor_identifier(&self) -> &#first_field_unnamed_type {
                <Value as #provider_identifier>::#accessor_identifier(*self)
            }
        }
    };
    generated.into()
}
