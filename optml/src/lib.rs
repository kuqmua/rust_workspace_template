struct ReplaceLts;
#[derive(newtype::Newtype)]
#[newtype(to_tokens)]
struct SynFieldTyWithStaticLts(syn::Type);
impl syn::visit_mut::VisitMut for ReplaceLts {
    fn visit_lifetime_mut(&mut self, i: &mut syn::Lifetime) {
        i.ident = syn::Ident::new(str_constants::text::STATIC, i.ident.span());
    }
}
#[proc_macro_derive(Optml)]
pub fn optml(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let generate_alignments_identifier_token_stream = |i: usize| {
        format!("alignments_{i}")
            .parse::<proc_macro2::TokenStream>()
            .expect("5a0bb723")
    };
    let di: syn::DeriveInput = syn::parse(input_token_stream).expect("a1d306de");
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
            let mut field_ty = field.ty.clone();
            let mut visitor = ReplaceLts;
            syn::visit_mut::VisitMut::visit_type_mut(&mut visitor, &mut field_ty);
            let field_ty_with_static_lts = SynFieldTyWithStaticLts(field_ty);
            quote::quote! {align_of::<#field_ty_with_static_lts>()}
        });
        let variant_info = variant.map_or_else(String::new, |variant_identifier| {
            format!("variant '{variant_identifier}' ")
        });
        let generate_or_copy_identifier = |field: &syn::Field, idx: usize| {
            field
                .ident
                .as_ref()
                .map_or_else(|| generate_field(idx), Clone::clone)
        };
        let assertions_token_stream = fields
            .iter()
            .zip(fields.iter().skip(1))
            .enumerate()
            .map(|(i, (field, next_field))| {
            let i_plus_one = i.saturating_add(1);
            let field_identifier = generate_or_copy_identifier(field, i);
            let field_next = generate_or_copy_identifier(next_field, i_plus_one);
            let message_token_stream = generate_quotes::dq_token_stream(&format!(
                "In {kind_name} '{identifier}' {variant_info}align_of field '{field_identifier}' < align_of field '{field_next}'. syn::Field '{field_next}' must be placed before '{field_identifier}' for better memory alignment",
            ));
            quote::quote! {
                assert!(
                    #alignments_token_stream[#i] >= #alignments_token_stream[#i_plus_one],
                    #message_token_stream,
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
                syn::Fields::Unnamed(fields) => &fields.unnamed,
                syn::Fields::Unit => {
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
                str_constants::text::STRUCT,
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
                        syn::Fields::Unnamed(fields) => &fields.unnamed,
                        syn::Fields::Unit => return None,
                    };
                    if fields.len() <= 1 {
                        return None;
                    }
                    generate_assertions_token_stream(
                        fields,
                        &generate_alignments_identifier_token_stream(i),
                        str_constants::text::ENUM,
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
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let has_only_lts = generics
        .params
        .iter()
        .all(|p| matches!(p, syn::GenericParam::Lifetime(_)));
    let (impl_token_stream, ty_token_stream) = if has_only_lts && !generics.params.is_empty() {
        let lts_count = generics.params.len();
        let undrscrs = std::iter::repeat_n(quote::quote! {'_}, lts_count);
        let new_ty_generics = quote::quote! {<#(#undrscrs),*>};
        (quote::quote! {}, new_ty_generics)
    } else {
        (
            quote::quote! { #impl_generics },
            quote::quote! { #ty_generics },
        )
    };
    let const_name_token_stream = quote::quote! {_OPTIMAL_PACK_CHECK};
    let impl_check_token_stream = quote::quote! {
        #[allow(unused_qualifications)]
        impl #impl_token_stream #identifier #ty_token_stream #where_clause {
            const #const_name_token_stream: () = {
                #ts
            };
        }
    };
    let has_type_parameters = generics
        .params
        .iter()
        .any(|p| matches!(p, syn::GenericParam::Type(_) | syn::GenericParam::Const(_)));
    let generated = if has_type_parameters {
        quote::quote! {#impl_check_token_stream}
    } else {
        quote::quote! {
            #impl_check_token_stream
            const _: () = #identifier::#const_name_token_stream;
        }
    };
    generated.into()
}
