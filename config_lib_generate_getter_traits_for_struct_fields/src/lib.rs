#[proc_macro_derive(GenerateGetterTraitsForStructFields)]
pub fn generate_getter_traits_for_struct_fields(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let di: syn::DeriveInput = syn::parse(input)
        .expect("49780295 generate_getter_traits_for_struct_fields invariant must hold");
    let identifier = &di.ident;
    let datastruct = match di.data {
        syn::Data::Struct(v) => v,
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("15cd72a2"),
    };
    let generated_traits_impls_token_stream = datastruct.fields.into_iter().map(|syn_field| {
        let field_type = &syn_field.ty;
        let field_identifier = syn_field
            .ident
            .as_ref()
            .expect("e5c23c45 generate_getter_traits_for_struct_fields invariant must hold");
        let upper_camel_case_field =
            naming_common::domain_types::ToTokensToUpperCamelCaseStr::case(&field_identifier);
        let trait_identifier = quote::format_ident!("Get{}", upper_camel_case_field);
        let fn_name_identifier = quote::format_ident!("get_{}", field_identifier);
        quote::quote! {
            impl app_state::#trait_identifier for #identifier {
                fn #fn_name_identifier (&self) -> &#field_type {
                    &self.#field_identifier
                }
            }
            impl app_state::#trait_identifier for &#identifier {
                fn #fn_name_identifier (&self) -> &#field_type {
                    &self.#field_identifier
                }
            }
        }
    });
    let generated = quote::quote! {#(#generated_traits_impls_token_stream)*};
    generated.into()
}
#[proc_macro_derive(GenerateGetterTrait)]
pub fn generate_getter_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let di: syn::DeriveInput =
        syn::parse(input).expect("195b48f5 generate_getter_trait invariant must hold");
    let identifier = &di.ident;
    let data_struct = match di.data {
        syn::Data::Struct(v) => v,
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("cd6bbc4e"),
    };
    let fields_unnamed = match data_struct.fields {
        syn::Fields::Unnamed(v) => v.unnamed,
        syn::Fields::Named(_) | syn::Fields::Unit => panic!("577cb86a"),
    };
    assert!(fields_unnamed.len() == 1, "1e82dc7e");
    let first_field_unnamed = fields_unnamed
        .iter()
        .next()
        .expect("7c2531fd generate_getter_trait invariant must hold");
    let first_field_unnamed_type = &first_field_unnamed.ty;
    let get_identifier_upper_camel_case =
        naming::domain_types::parameter::GetSelfUpperCamelCase::from_tokens(&identifier);
    let get_identifier_snake_case =
        naming::domain_types::parameter::GetSelfSnakeCase::from_tokens(&identifier);
    let generated = quote::quote! {
        pub trait #get_identifier_upper_camel_case {
            fn #get_identifier_snake_case(&self) -> &#first_field_unnamed_type;
        }
        impl<Value> #get_identifier_upper_camel_case for &Value
        where
            Value: #get_identifier_upper_camel_case + ?Sized,
        {
            fn #get_identifier_snake_case(&self) -> &#first_field_unnamed_type {
                <Value as #get_identifier_upper_camel_case>::#get_identifier_snake_case(*self)
            }
        }
    };
    generated.into()
}
