#[derive(Debug, Clone, Copy)]
enum GeneratedItemKind {
    Enum,
    Struct,
}

struct ReplaceLts;

impl syn::visit_mut::VisitMut for ReplaceLts {
    fn visit_lifetime_mut(&mut self, i: &mut syn::Lifetime) {
        i.ident = syn::Ident::new("static", i.ident.span());
    }
}

fn generate_field_identifier(
    field: &syn::Field,
    unnamed_field_identifier: syn::Ident,
) -> syn::Ident {
    field
        .ident
        .as_ref()
        .map_or(unnamed_field_identifier, Clone::clone)
}

fn generate_assertions_token_stream(
    fields: &[&syn::Field],
    alignments_token_stream: &dyn quote::ToTokens,
    generated_item_kind: GeneratedItemKind,
    variant: Option<&syn::Ident>,
    ident: &syn::Ident,
) -> Option<proc_macro2::TokenStream> {
    let fields_len = fields.len();
    if fields_len <= 1 {
        return None;
    }
    let align_of_ts = fields.iter().copied().map(|field| {
        let mut ty = field.ty.clone();
        let mut visitor = ReplaceLts;
        syn::visit_mut::VisitMut::visit_type_mut(&mut visitor, &mut ty);
        quote::quote! {align_of::<#ty>()}
    });
    let variant_info =
        variant.map_or_else(String::new, |variant_ident| format!("variant '{variant_ident}' "));
    let generated_item_kind_name = match generated_item_kind {
        GeneratedItemKind::Enum => "enum",
        GeneratedItemKind::Struct => "struct",
    };
    let assertions_ts = fields
        .iter()
        .copied()
        .zip(fields.iter().copied().skip(1))
        .enumerate()
        .map(|(i, (field, next_field))| {
            let i_plus_one = i.saturating_add(1);
            let fi = generate_field_identifier(
                field,
                syn::Ident::new(&format!("field_{i}"), ident.span()),
            );
            let fi_next = generate_field_identifier(
                next_field,
                syn::Ident::new(&format!("field_{i_plus_one}"), ident.span()),
            );
            let message_literal = ::syn::LitStr::new(
                &format!(
                    "In {generated_item_kind_name} '{ident}' {variant_info}align_of field '{fi}' \
                     < align_of field '{fi_next}'. Field '{fi_next}' must be placed before '{fi}' \
                     for better memory alignment",
                ),
                ::proc_macro2::Span::call_site(),
            );
            let msg_ts = quote::ToTokens::into_token_stream(&message_literal);
            let assert_ident =
                ::proc_macro2::Ident::new("assert", ::proc_macro2::Span::call_site());
            quote::quote! {
                #assert_ident!(
                    #alignments_token_stream[#i] >= #alignments_token_stream[#i_plus_one],
                    #msg_ts,
                );
            }
        });
    Some(quote::quote! {
        let #alignments_token_stream: [usize; #fields_len] = [#(#align_of_ts),*];
        #(#assertions_ts)*
    })
}

fn build_generated_token_stream(
    ident: &syn::Ident,
    generics: &syn::Generics,
    token_stream: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let has_only_lifetimes = generics
        .params
        .iter()
        .all(|param| matches!(param, syn::GenericParam::Lifetime(_)));
    let (impl_token_stream, ty_token_stream) = if has_only_lifetimes && !generics.params.is_empty()
    {
        let lifetimes_count = generics.params.len();
        let underscores = core::iter::repeat_n(quote::quote! {'_}, lifetimes_count);
        let new_ty_generics = quote::quote! {<#(#underscores),*>};
        (quote::quote! {}, new_ty_generics)
    } else {
        (quote::quote! { #impl_generics }, quote::quote! { #ty_generics })
    };
    let const_name_ts = quote::quote! {_OPTIMAL_PACK_CHECK};
    let allow_ident = ::proc_macro2::Ident::new("allow", ::proc_macro2::Span::call_site());
    let unused_qualifications_ident =
        ::proc_macro2::Ident::new("unused_qualifications", ::proc_macro2::Span::call_site());
    let impl_check_token_stream = quote::quote! {
        #[#allow_ident(#unused_qualifications_ident)]
        impl #impl_token_stream #ident #ty_token_stream #where_clause {
            const #const_name_ts: () = {
                #token_stream
            };
        }
    };
    let has_type_parameters = generics
        .params
        .iter()
        .any(|param| matches!(param, syn::GenericParam::Type(_) | syn::GenericParam::Const(_)));
    if has_type_parameters {
        return quote::quote! {#impl_check_token_stream};
    }
    quote::quote! {
        #impl_check_token_stream
        const _: () = #ident::#const_name_ts;
    }
}

#[proc_macro_derive(Optml)]
pub fn optml(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: syn::DeriveInput = match syn::parse(input_token_stream) {
        Ok(derive_input) => derive_input,
        Err(err) => return err.to_compile_error().into(),
    };
    let syn::DeriveInput {
        ident,
        generics,
        data,
        ..
    } = derive_input;
    let generate_alignments_identifier_token_stream = |index: usize| {
        quote::ToTokens::into_token_stream(syn::Ident::new(
            &format!("alignments_{index}"),
            ident.span(),
        ))
    };
    let token_stream = match data {
        syn::Data::Struct(data_struct) => {
            let fields = data_struct.fields.iter().collect::<Vec<&syn::Field>>();
            let fields_len = fields.len();
            if fields_len <= 1 {
                return proc_macro::TokenStream::new();
            }
            match generate_assertions_token_stream(
                &fields,
                &quote::quote! {alignments},
                GeneratedItemKind::Struct,
                None,
                &ident,
            ) {
                Some(assertions) => assertions,
                None => {
                    return proc_macro::TokenStream::new();
                }
            }
        }
        syn::Data::Enum(data_enum) => {
            let mut variants_token_stream = Vec::new();
            for (variant_index, variant) in data_enum.variants.iter().enumerate() {
                let variant_ident = &variant.ident;
                let fields = variant.fields.iter().collect::<Vec<&syn::Field>>();
                let fields_len = fields.len();
                if fields_len <= 1 {
                    continue;
                }
                if let Some(assertions) = generate_assertions_token_stream(
                    &fields,
                    &generate_alignments_identifier_token_stream(variant_index),
                    GeneratedItemKind::Enum,
                    Some(variant_ident),
                    &ident,
                ) {
                    variants_token_stream.push(assertions);
                }
            }
            if variants_token_stream.is_empty() {
                return proc_macro::TokenStream::new();
            }
            quote::quote! {#(#variants_token_stream)*}
        }
        syn::Data::Union(_) => {
            return proc_macro::TokenStream::new();
        }
    };
    if generics.params.is_empty() {
        return build_generated_token_stream(&ident, &generics, &token_stream).into();
    }
    build_generated_token_stream(&ident, &generics, &token_stream).into()
}
