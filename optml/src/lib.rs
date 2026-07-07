struct ReplaceLts;
#[derive(Clone, Copy)]
struct OptmlSynField<'field_lt>(&'field_lt syn::Field);
struct FieldTyWithStaticLts(syn::Type);
struct AlignOfTs(proc_macro2::TokenStream);
impl quote::ToTokens for FieldTyWithStaticLts {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
impl quote::ToTokens for AlignOfTs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
impl syn::visit_mut::VisitMut for ReplaceLts {
    fn visit_lifetime_mut(&mut self, i: &mut syn::Lifetime) {
        i.ident = syn::Ident::new("static", i.ident.span());
    }
}
#[allow(clippy::single_call_fn)] // isolated helper keeps lifetime rewrite reusable when alignment logic grows
fn field_ty_with_static_lts(field: OptmlSynField<'_>) -> FieldTyWithStaticLts {
    let mut ft = field.0.ty.clone();
    let mut visitor = ReplaceLts;
    syn::visit_mut::VisitMut::visit_type_mut(&mut visitor, &mut ft);
    FieldTyWithStaticLts(ft)
}
#[allow(clippy::single_call_fn)] // isolated helper keeps align token generation reusable and explicit
fn gen_align_of_ts(field: OptmlSynField<'_>) -> AlignOfTs {
    let ft = field_ty_with_static_lts(field);
    AlignOfTs(quote::quote! {align_of::<#ft>()})
}
#[proc_macro_derive(Optml)]
pub fn optml(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let gen_alignments_ident_ts = |i: usize| {
        format!("alignments_{i}")
            .parse::<proc_macro2::TokenStream>()
            .expect("5a0bb723")
    };
    let di: syn::DeriveInput = syn::parse(input_ts).expect("a1d306de");
    let ident = &di.ident;
    let gen_fi = |i: usize| syn::Ident::new(&format!("field_{i}"), ident.span());
    let gen_assertions_ts = |fields: &syn::punctuated::Punctuated<
        syn::Field,
        syn::token::Comma,
    >,
                             alignments_ts: &dyn quote::ToTokens,
                             kind_name: &'static str,
                             variant: Option<&syn::Ident>|
     -> Option<proc_macro2::TokenStream> {
        let fields_len = fields.len();
        if fields_len <= 1 {
            return None;
        }
        let align_of_ts = fields
            .iter()
            .map(|field| gen_align_of_ts(OptmlSynField(field)));
        let variant_info = variant.map_or_else(String::new, |variant_ident| {
            format!("variant '{variant_ident}' ")
        });
        let gen_or_copy_ident = |field: &syn::Field, idx: usize| {
            field
                .ident
                .as_ref()
                .map_or_else(|| gen_fi(idx), Clone::clone)
        };
        let assertions_ts = fields
            .iter()
            .zip(fields.iter().skip(1))
            .enumerate()
            .map(|(i, (field, next_field))| {
            let i_plus_one = i.saturating_add(1);
            let fi = gen_or_copy_ident(field, i);
            let fi_next = gen_or_copy_ident(next_field, i_plus_one);
            let msg_ts = gen_quotes::dq_ts(&format!(
                "In {kind_name} '{ident}' {variant_info}align_of field '{fi}' < align_of field '{fi_next}'. syn::Field '{fi_next}' must be placed before '{fi}' for better memory alignment",
            ));
            quote::quote! {
                assert!(
                    #alignments_ts[#i] >= #alignments_ts[#i_plus_one],
                    #msg_ts,
                );
            }
        });
        Some(quote::quote! {
            let #alignments_ts: [usize; #fields_len] = [#(#align_of_ts),*];
            #(#assertions_ts)*
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
            match gen_assertions_ts(fields, &quote::quote! {alignments}, "struct", None) {
                Some(v) => v,
                None => {
                    return proc_macro::TokenStream::new();
                }
            }
        }
        syn::Data::Enum(data_enum) => {
            let vars_ts = data_enum
                .variants
                .iter()
                .enumerate()
                .filter_map(|(i, var)| {
                    let var_ident = &var.ident;
                    let fields = match &var.fields {
                        syn::Fields::Named(fields) => &fields.named,
                        syn::Fields::Unnamed(fields) => &fields.unnamed,
                        syn::Fields::Unit => return None,
                    };
                    if fields.len() <= 1 {
                        return None;
                    }
                    gen_assertions_ts(fields, &gen_alignments_ident_ts(i), "enum", Some(var_ident))
                })
                .collect::<Vec<proc_macro2::TokenStream>>();
            if vars_ts.is_empty() {
                return proc_macro::TokenStream::new();
            }
            quote::quote! {#(#vars_ts)*}
        }
        syn::Data::Union(_) => {
            return proc_macro::TokenStream::new();
        }
    };
    let generics = &di.generics;
    let (impl_generics, ty_generics, wh_clause) = generics.split_for_impl();
    let has_only_lts = generics
        .params
        .iter()
        .all(|p| matches!(p, syn::GenericParam::Lifetime(_)));
    let (impl_ts, ty_ts) = if has_only_lts && !generics.params.is_empty() {
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
    let const_name_ts = quote::quote! {_OPTIMAL_PACK_CHECK};
    let impl_check_ts = quote::quote! {
        #[allow(unused_qualifications)]
        impl #impl_ts #ident #ty_ts #wh_clause {
            const #const_name_ts: () = {
                #ts
            };
        }
    };
    let has_type_prms = generics
        .params
        .iter()
        .any(|p| matches!(p, syn::GenericParam::Type(_) | syn::GenericParam::Const(_)));
    let generated = if has_type_prms {
        quote::quote! {#impl_check_ts}
    } else {
        quote::quote! {
            #impl_check_ts
            const _: () = #ident::#const_name_ts;
        }
    };
    generated.into()
}
