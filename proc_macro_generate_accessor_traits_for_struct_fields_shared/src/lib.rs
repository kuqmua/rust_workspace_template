#![allow(
    clippy::must_use_candidate,
    clippy::useless_conversion,
    reason = "shared proc-macro implementations preserve original entrypoint conversion points while returning proc_macro2 streams to one-entrypoint facade crates; every result is consumed immediately by its facade"
)]

#[cfg(test)]
mod test_accessor_generics;

pub fn generate_accessor_traits_for_struct_fields(
    token_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    panic_location::panic_location();
    let derive_input: syn::DeriveInput =
        syn::parse2(token_stream).expect(constants_str::DIAGNOSTIC_49780295);
    let identifier = &derive_input.ident;
    let (impl_generics, type_generics, where_clause) = derive_input.generics.split_for_impl();
    let datastruct = match derive_input.data {
        syn::Data::Struct(v) => v,
        syn::Data::Enum(_) | syn::Data::Union(_) => {
            std::panic::panic_any(constants_str::PANIC_15CD72A2)
        }
    };
    let generated_traits_impls_result = datastruct.fields.into_iter().map(|syn_field| {
        let field_type = &syn_field.ty;
        let field_identifier = syn_field
            .ident
            .as_ref()
            .expect(constants_str::DIAGNOSTIC_E5C23C45);
        let upper_camel_case_field =
            naming_common::domain_types::ToTokensToUpperCamelCaseStr::try_case(
                &syn::ext::IdentExt::unraw(field_identifier),
            )
            .map_err(|error| syn::Error::new_spanned(field_identifier, error))?;
        let trait_identifier = quote::format_ident!("{}Provider", upper_camel_case_field);
        Ok(quote::quote! {
            impl #impl_generics app_state::#trait_identifier for #identifier #type_generics #where_clause {
                fn #field_identifier (&self) -> &#field_type {
                    &self.#field_identifier
                }
            }
            impl #impl_generics app_state::#trait_identifier for &#identifier #type_generics #where_clause {
                fn #field_identifier (&self) -> &#field_type {
                    &self.#field_identifier
                }
            }
        })
    }).collect::<syn::Result<Vec<proc_macro2::TokenStream>>>();
    let generated = match generated_traits_impls_result {
        Ok(generated_traits_impls_token_stream) => {
            quote::quote! {#(#generated_traits_impls_token_stream)*}
        }
        Err(error) => error.into_compile_error(),
    };
    generated.into()
}
pub fn generate_accessor_trait(token_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    panic_location::panic_location();
    let derive_input: syn::DeriveInput =
        syn::parse2(token_stream).expect(constants_str::DIAGNOSTIC_195B48F5);
    let mut pending_tokens = quote::ToTokens::to_token_stream(&derive_input)
        .into_iter()
        .collect::<Vec<_>>();
    let mut input_identifiers = Vec::new();
    while let Some(token) = pending_tokens.pop() {
        match token {
            proc_macro2::TokenTree::Ident(input_identifier) => {
                input_identifiers.push(syn::ext::IdentExt::unraw(&input_identifier));
            }
            proc_macro2::TokenTree::Group(group) => pending_tokens.extend(group.stream()),
            proc_macro2::TokenTree::Punct(_) | proc_macro2::TokenTree::Literal(_) => {}
        }
    }
    let identifier = &derive_input.ident;
    let data_struct = match derive_input.data {
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
    let accessor_identifier = match naming_common::domain_types::ToTokensToSnakeCaseStr::try_case(
        &syn::ext::IdentExt::unraw(identifier),
    ) {
        Ok(case_string) => quote::format_ident!("{}", case_string),
        Err(error) => return syn::Error::new_spanned(identifier, error).into_compile_error(),
    };
    let (_, type_generics, input_where_clause) = derive_input.generics.split_for_impl();
    let trait_generics = &derive_input.generics;
    let Some(forwarded_identifier) = (0usize..=input_identifiers.len())
        .map(|index| quote::format_ident!("{identifier}{index}"))
        .find(|candidate| !input_identifiers.contains(candidate))
    else {
        return syn::Error::new_spanned(&derive_input.generics, constants_str::DUPLICATE)
            .into_compile_error();
    };
    let mut forwarding_generics = derive_input.generics.clone();
    forwarding_generics
        .params
        .push(syn::parse_quote!(#forwarded_identifier: ?Sized));
    forwarding_generics
        .make_where_clause()
        .predicates
        .push(syn::parse_quote!(#forwarded_identifier: #provider_identifier #type_generics));
    let (forwarding_impl_generics, _, forwarding_where_clause) =
        forwarding_generics.split_for_impl();
    let generated = quote::quote! {
        pub trait #provider_identifier #trait_generics #input_where_clause {
            fn #accessor_identifier(&self) -> &#first_field_unnamed_type;
        }
        impl #forwarding_impl_generics #provider_identifier #type_generics for &#forwarded_identifier
        #forwarding_where_clause
        {
            fn #accessor_identifier(&self) -> &#first_field_unnamed_type {
                <#forwarded_identifier as #provider_identifier #type_generics>::#accessor_identifier(*self)
            }
        }
    };
    generated.into()
}
