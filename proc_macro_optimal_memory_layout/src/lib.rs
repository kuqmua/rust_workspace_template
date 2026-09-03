#[proc_macro_derive(OptimalMemoryLayout, attributes(optimal_memory_layout))]
#[allow(
    clippy::useless_concat,
    reason = "the foundational proc macro cannot depend on constants_str without creating a dependency cycle"
)]
pub fn optimal_memory_layout(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let generate_alignments_identifier_token_stream = |i: usize| {
        proc_macro2::Ident::new(
            format!("alignments_{i}").as_str(),
            proc_macro2::Span::call_site(),
        )
    };
    let di: syn::DeriveInput = match syn::parse(token_stream) {
        Ok(derive_input) => derive_input,
        Err(error) => return error.to_compile_error().into(),
    };
    let mut skip = false;
    let attribute_result = di
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident(stringify!(optimal_memory_layout)))
        .try_for_each(|attr| {
            attr.parse_nested_meta(|metadata| {
                if metadata.path.is_ident(stringify!(skip)) {
                    skip = true;
                    Ok(())
                } else {
                    Err(metadata.error(concat!(
                        "6e9230ab unsupported optimal_memory_layout attribute"
                    )))
                }
            })
        });
    if let Err(error) = attribute_result {
        return error.to_compile_error().into();
    }
    if skip {
        return proc_macro::TokenStream::new();
    }
    let identifier = &di.ident;
    let generate_field = |i: usize| syn::Ident::new(&format!("field_{i}"), identifier.span());
    let generate_assertions_token_stream = |fields: &syn::punctuated::Punctuated<
        syn::Field,
        syn::token::Comma,
    >,
                                            alignments_token_stream: &dyn quote::ToTokens,
                                            kind_name: &'static str,
                                            variant: Option<&syn::Ident>|
     -> Option<proc_macro2::TokenStream> {
        let fields_len = fields.len();
        if fields_len <= 1 {
            return None;
        }
        let align_of_token_stream = fields.iter().map(|field| {
            let field_ty = &field.ty;
            quote::quote! {align_of::<#field_ty>()}
        });
        let variant_info = variant.map_or_else(String::new, |variant_identifier| {
            format!("variant '{variant_identifier}' ")
        });
        let generate_or_copy_identifier = |field: &syn::Field, index: usize| {
            field
                .ident
                .as_ref()
                .map_or_else(|| generate_field(index), Clone::clone)
        };
        let assertions_token_stream = fields
            .iter()
            .zip(fields.iter().skip(1))
            .enumerate()
            .map(|(i, (field, next_field))| {
            let i_plus_one = i.saturating_add(1);
            let field_identifier = generate_or_copy_identifier(field, i);
            let field_next = generate_or_copy_identifier(next_field, i_plus_one);
            let message = syn::LitStr::new(&format!(
                "In {kind_name} '{identifier}' {variant_info}align_of field '{field_identifier}' < align_of field '{field_next}'. syn::Field '{field_next}' must be placed before '{field_identifier}' for better memory alignment",
            ), identifier.span());
            quote::quote! {
                assert!(
                    #alignments_token_stream[#i] >= #alignments_token_stream[#i_plus_one],
                    #message,
                );
            }
        });
        Some(quote::quote! {
            let #alignments_token_stream: [usize; #fields_len] = [#(#align_of_token_stream),*];
            #(#assertions_token_stream)*
        })
    };
    let ts = match &di.data {
        syn::Data::Struct(data) => {
            let fields = match &data.fields {
                syn::Fields::Named(fields) => &fields.named,
                syn::Fields::Unnamed(_) | syn::Fields::Unit => {
                    return proc_macro::TokenStream::new();
                }
            };
            let fields_len = fields.len();
            if fields_len <= 1 {
                return proc_macro::TokenStream::new();
            }
            match generate_assertions_token_stream(
                fields,
                &quote::quote! {alignments},
                stringify!(struct),
                None,
            ) {
                Some(v) => v,
                None => {
                    return proc_macro::TokenStream::new();
                }
            }
        }
        syn::Data::Enum(data_enum) => {
            let vars_token_stream = data_enum
                .variants
                .iter()
                .enumerate()
                .filter_map(|(i, var)| {
                    let var_identifier = &var.ident;
                    let fields = match &var.fields {
                        syn::Fields::Named(fields) => &fields.named,
                        syn::Fields::Unnamed(_) | syn::Fields::Unit => return None,
                    };
                    if fields.len() <= 1 {
                        return None;
                    }
                    generate_assertions_token_stream(
                        fields,
                        &generate_alignments_identifier_token_stream(i),
                        stringify!(enum),
                        Some(var_identifier),
                    )
                })
                .collect::<Vec<proc_macro2::TokenStream>>();
            if vars_token_stream.is_empty() {
                return proc_macro::TokenStream::new();
            }
            quote::quote! {#(#vars_token_stream)*}
        }
        syn::Data::Union(_) => {
            return proc_macro::TokenStream::new();
        }
    };
    let generics = &di.generics;
    if !generics.params.is_empty() {
        return proc_macro::TokenStream::new();
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let const_name_token_stream = quote::quote! {_OPTIMAL_PACK_CHECK};
    let impl_check_token_stream = quote::quote! {
        #[cfg(not(target_arch = "wasm32"))]

        #[allow(unused_qualifications, reason = "lint suppression is required here")]
        impl #impl_generics #identifier #ty_generics #where_clause {
            const #const_name_token_stream: () = {
                #ts
            };
        }
    };
    let generated = quote::quote! {
        #impl_check_token_stream
        #[cfg(not(target_arch = "wasm32"))]
        const _: () = #identifier::#const_name_token_stream;
    };
    generated.into()
}
