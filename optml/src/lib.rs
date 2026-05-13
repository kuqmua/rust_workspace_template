use core::iter::repeat_n;

use proc_macro::TokenStream as Ts;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Field, GenericParam, Ident, parse, visit_mut::VisitMut};
struct ReplaceLts;
impl VisitMut for ReplaceLts {
    fn visit_lifetime_mut(&mut self, i: &mut syn::Lifetime) {
        i.ident = Ident::new("static", i.ident.span());
    }
}

fn generate_field_identifier(field: &Field, index: usize, ident: &Ident) -> Ident {
    field
        .ident
        .as_ref()
        .map_or_else(|| Ident::new(&format!("field_{index}"), ident.span()), Clone::clone)
}

fn generate_assertions_token_stream(
    fields: &[&Field],
    alignments_token_stream: &dyn ToTokens,
    kind_name: &'static str,
    variant: Option<&Ident>,
    ident: &Ident,
) -> Option<Ts2> {
    let fields_len = fields.len();
    if fields_len <= 1 {
        return None;
    }
    let align_of_ts = fields.iter().copied().map(|field| {
        let mut ty = field.ty.clone();
        let mut visitor = ReplaceLts;
        visitor.visit_type_mut(&mut ty);
        quote! {align_of::<#ty>()}
    });
    let variant_info =
        variant.map_or_else(String::new, |variant_ident| format!("variant '{variant_ident}' "));
    let assertions_ts = fields
        .iter()
        .copied()
        .zip(fields.iter().copied().skip(1))
        .enumerate()
        .map(|(i, (field, next_field))| {
            let i_plus_one = i.saturating_add(1);
            let fi = generate_field_identifier(field, i, ident);
            let fi_next = generate_field_identifier(next_field, i_plus_one, ident);
            let msg_ts = ::syn::LitStr::new(
                &format!(
                    "In {kind_name} '{ident}' {variant_info}align_of field '{fi}' < align_of \
                     field '{fi_next}'. Field '{fi_next}' must be placed before '{fi}' for better \
                     memory alignment",
                ),
                ::proc_macro2::Span::call_site(),
            )
            .into_token_stream();
            let assert_ident =
                ::proc_macro2::Ident::new("assert", ::proc_macro2::Span::call_site());
            quote! {
                #assert_ident!(
                    #alignments_token_stream[#i] >= #alignments_token_stream[#i_plus_one],
                    #msg_ts,
                );
            }
        });
    Some(quote! {
        let #alignments_token_stream: [usize; #fields_len] = [#(#align_of_ts),*];
        #(#assertions_ts)*
    })
}

fn build_generated_token_stream(ident: &Ident, generics: &syn::Generics, ts: &Ts2) -> Ts2 {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let has_only_lifetimes = generics
        .params
        .iter()
        .all(|param| matches!(param, GenericParam::Lifetime(_)));
    let (impl_token_stream, ty_token_stream) = if has_only_lifetimes && !generics.params.is_empty()
    {
        let lifetimes_count = generics.params.len();
        let underscores = repeat_n(quote! {'_}, lifetimes_count);
        let new_ty_generics = quote! {<#(#underscores),*>};
        (quote! {}, new_ty_generics)
    } else {
        (quote! { #impl_generics }, quote! { #ty_generics })
    };
    let const_name_ts = quote! {_OPTIMAL_PACK_CHECK};
    let allow_ident = ::proc_macro2::Ident::new("allow", ::proc_macro2::Span::call_site());
    let unused_qualifications_ident =
        ::proc_macro2::Ident::new("unused_qualifications", ::proc_macro2::Span::call_site());
    let impl_check_token_stream = quote! {
        #[#allow_ident(#unused_qualifications_ident)]
        impl #impl_token_stream #ident #ty_token_stream #where_clause {
            const #const_name_ts: () = {
                #ts
            };
        }
    };
    let has_type_parameters = generics
        .params
        .iter()
        .any(|param| matches!(param, GenericParam::Type(_) | GenericParam::Const(_)));
    if has_type_parameters {
        return quote! {#impl_check_token_stream};
    }
    quote! {
        #impl_check_token_stream
        const _: () = #ident::#const_name_ts;
    }
}

#[proc_macro_derive(Optml)]
pub fn optml(input_ts: Ts) -> Ts {
    let derive_input: DeriveInput = match parse(input_ts) {
        Ok(derive_input) => derive_input,
        Err(err) => return err.to_compile_error().into(),
    };
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = derive_input;
    let gen_alignments_ident_ts =
        |i: usize| Ident::new(&format!("alignments_{i}"), ident.span()).into_token_stream();
    let ts = match data {
        Data::Struct(data_struct) => {
            let fields = data_struct.fields.iter().collect::<Vec<&Field>>();
            let fields_len = fields.len();
            if fields_len <= 1 {
                return Ts::new();
            }
            match generate_assertions_token_stream(
                &fields,
                &quote! {alignments},
                "struct",
                None,
                &ident,
            ) {
                Some(assertions) => assertions,
                None => {
                    return Ts::new();
                }
            }
        }
        Data::Enum(data_enum) => {
            let mut vars_ts = Vec::new();
            for (var_idx, var) in data_enum.variants.iter().enumerate() {
                let var_ident = &var.ident;
                let fields = var.fields.iter().collect::<Vec<&Field>>();
                let fields_len = fields.len();
                if fields_len <= 1 {
                    continue;
                }
                if let Some(assertions) = generate_assertions_token_stream(
                    &fields,
                    &gen_alignments_ident_ts(var_idx),
                    "enum",
                    Some(var_ident),
                    &ident,
                ) {
                    vars_ts.push(assertions);
                }
            }
            if vars_ts.is_empty() {
                return Ts::new();
            }
            quote! {#(#vars_ts)*}
        }
        Data::Union(_) => {
            return Ts::new();
        }
    };
    if generics.params.is_empty() {
        return build_generated_token_stream(&ident, &generics, &ts).into();
    }
    build_generated_token_stream(&ident, &generics, &ts).into()
}
