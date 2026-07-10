const SC_STRING_MAX_LEN: usize = 1_048_576;
#[derive(Clone, Copy)]
struct ToScInput<'input_lt>(&'input_lt str);
#[derive(newtype::BoundedString)]
#[bounded_string(max = SC_STRING_MAX_LEN, description = "snake case string")]
struct ScString(String);
#[allow(clippy::single_call_fn)] // extracted to isolate case-normalization logic and keep macro expansion flow focused
fn to_sc(input: ToScInput<'_>) -> ScString {
    let normalized = input
        .0
        .split(|ch| !char::is_alphanumeric(ch))
        .filter(|part| !part.is_empty())
        .collect::<Vec<&str>>()
        .join(" ");
    ScString::try_from(naming_cmn::AsRefStrToScStr::case(&normalized))
        .unwrap_or_else(ScString::from)
}
#[proc_macro]
pub fn gen_derive_ts_builder(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Clone, optml::Optml)]
    struct El {
        d_trait_name_if_sc: proc_macro2::TokenStream,
        d_trait_name_sc: proc_macro2::TokenStream,
        d_trait_name_ucc: proc_macro2::TokenStream,
        trait_type: proc_macro2::TokenStream,
    }
    let make_pub_sc_ts = quote::quote! {make_pub};
    let make_pub_if_sc_ts = quote::quote! {make_pub_if};
    let make_pub_ucc_ts = quote::quote! {MakePub};
    let el_vec = serde_json::from_str::<Vec<String>>(&input_ts.to_string())
        .expect("c5d09740")
        .into_iter()
        .map(|el| {
            let sc = to_sc(ToScInput(&el));
            El {
                d_trait_name_ucc: {
                    let v = naming::prm::DSelfUcc::from_display(&sc.0);
                    quote::quote! {#v}
                },
                d_trait_name_sc: {
                    let v = naming::prm::DSelfSc::from_display(&sc.0);
                    quote::quote! {#v}
                },
                d_trait_name_if_sc: {
                    let v = naming::prm::DSelfIfSc::from_display(&sc.0);
                    quote::quote! {#v}
                },
                trait_type: el.parse::<proc_macro2::TokenStream>().expect("8672240f"),
            }
        })
        .collect::<Vec<El>>();
    let (make_pub_pub_enum_ts, pub_enum_derive_vec_ts) = {
        fn gen_ts(ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
            quote::quote! {
                #[derive(Debug, Clone, Copy, optml::Optml)]
                pub enum #ident {
                    True,
                    False
                }
            }
        }
        (
            gen_ts(&make_pub_ucc_ts),
            el_vec.iter().map(|el| gen_ts(&el.d_trait_name_ucc)),
        )
    };
    let (make_pub_derive_trait_name_bool_ts, field_vec_ts) = {
        fn gen_ts(ident: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
            quote::quote! {#ident: bool,}
        }
        (
            gen_ts(&make_pub_sc_ts),
            el_vec.iter().map(|el| gen_ts(&el.d_trait_name_sc)),
        )
    };
    let (make_pub_derive_and_derive_if_ts, derive_and_derive_if_vec_ts) = {
        let gen_ts = |first_name_ts: &dyn quote::ToTokens,
                      second_name_ts: &dyn quote::ToTokens,
                      condition_type_ts: &dyn quote::ToTokens| {
            quote::quote! {
                pub const fn #first_name_ts(mut self) -> Self {
                    self.#first_name_ts = true;
                    self
                }
                pub const fn #second_name_ts(mut self, condition: #condition_type_ts) -> Self {
                    if let #condition_type_ts::True = condition {
                        self.#first_name_ts = true;
                    }
                    self
                }
            }
        };
        (
            gen_ts(&make_pub_sc_ts, &make_pub_if_sc_ts, &make_pub_ucc_ts),
            {
                let ts = el_vec.iter().map(|el| {
                    gen_ts(
                        &el.d_trait_name_sc,
                        &el.d_trait_name_if_sc,
                        &el.d_trait_name_ucc,
                    )
                });
                quote::quote! {#(#ts)*}
            },
        )
    };
    let if_self_derive_acc_push_vec_ts = el_vec.iter().map(|el| {
        let d_trait_name_sc = &el.d_trait_name_sc;
        let trait_type = &el.trait_type;
        quote::quote! {
            if self.#d_trait_name_sc {
                acc_2a71375c.push(quote::quote!{#trait_type});
            }
        }
    });
    let derive_ts_builder_ucc = quote::quote! {DTsBuilder};
    let struct_or_enum_ucc = quote::quote! {StructOrEnum};
    let quote_to_tokens_ts = quote::quote! {quote::ToTokens};
    let ts2_ts = quote::quote! {proc_macro2::TokenStream};
    let generated: proc_macro2::TokenStream = quote::quote! {
        #make_pub_pub_enum_ts
        #(#pub_enum_derive_vec_ts)*
        #[derive(Debug, Clone, Copy, optml::Optml)]
        enum #struct_or_enum_ucc {
            Struct,
            Enum
        }
        #[derive(Debug, Default, Clone, Copy, optml::Optml)]
        pub struct #derive_ts_builder_ucc {
            #make_pub_derive_trait_name_bool_ts
            #(#field_vec_ts)*
        }
        impl #derive_ts_builder_ucc {
            pub fn new() -> Self {
                Self::default()
            }
            #make_pub_derive_and_derive_if_ts
            #derive_and_derive_if_vec_ts
            fn build_h(
                self,
                struct_or_enum: #struct_or_enum_ucc,
                ann: &dyn #quote_to_tokens_ts,
                ident_d8cbb733: &dyn #quote_to_tokens_ts,
                generics_7d48c97a: &dyn #quote_to_tokens_ts,
                ts: &dyn #quote_to_tokens_ts,
            ) -> #ts2_ts {
                let mb_pub_ts = self.#make_pub_sc_ts.then(|| quote::quote!{pub});
                let derive_ts = {
                    let mut acc_2a71375c = Vec::new();
                    #(#if_self_derive_acc_push_vec_ts)*
                    acc_2a71375c
                };
                let struct_or_enum_ts = match struct_or_enum {
                    #struct_or_enum_ucc::Struct => quote::quote!{struct},
                    #struct_or_enum_ucc::Enum => quote::quote!{enum},
                };
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_pound(&mut _s);
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Bracket,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "derive");
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    {
                                        use ::quote::__private::ext::*;
                                        let mut _i = 0usize;
                                        let has_iter = ::quote::__private::HasIterator::<false>;
                                        #[allow(unused_mut)]
                                        let (mut derive_ts, i) = derive_ts
                                            .quote_into_iter();
                                        let has_iter = has_iter | i;
                                        <_ as ::quote::__private::CheckHasIterator<
                                            true,
                                        >>::check(has_iter);
                                        loop {
                                            let derive_ts = match derive_ts.next() {
                                                Some(_x) => ::quote::__private::RepInterp(_x),
                                                None => break,
                                            };
                                            if _i > 0 {
                                                ::quote::__private::push_comma(&mut _s);
                                            }
                                            _i += 1;
                                            ::#quote_to_tokens_ts::to_tokens(&derive_ts, &mut _s);
                                        }
                                    }
                                    _s
                                },
                            );
                            _s
                        },
                    );
                    ::#quote_to_tokens_ts::to_tokens(&ann, &mut _s);
                    ::#quote_to_tokens_ts::to_tokens(&mb_pub_ts, &mut _s);
                    ::#quote_to_tokens_ts::to_tokens(&struct_or_enum_ts, &mut _s);
                    ::#quote_to_tokens_ts::to_tokens(&ident_d8cbb733, &mut _s);
                    ::#quote_to_tokens_ts::to_tokens(&generics_7d48c97a, &mut _s);
                    ::#quote_to_tokens_ts::to_tokens(&ts, &mut _s);
                    _s
                }
            }
            pub fn build_struct(
                self,
                ann: &dyn #quote_to_tokens_ts,
                ident_d87c6809: &dyn #quote_to_tokens_ts,
                generics_c33a0ef2: &dyn #quote_to_tokens_ts,
                ts: &dyn #quote_to_tokens_ts,
            ) -> #ts2_ts {
                self.build_h(
                    #struct_or_enum_ucc::Struct,
                    ann,
                    ident_d87c6809,
                    generics_c33a0ef2,
                    ts
                )
            }
            pub fn build_enum(
                self,
                ann: &dyn #quote_to_tokens_ts,
                ident_273dd063: &dyn #quote_to_tokens_ts,
                generics_84bc3f7f: &dyn #quote_to_tokens_ts,
                ts: &dyn #quote_to_tokens_ts,
            ) -> #ts2_ts {
                self.build_h(
                    #struct_or_enum_ucc::Enum,
                    ann,
                    ident_273dd063,
                    generics_84bc3f7f,
                    ts
                )
            }
        }
    };
    generated.into()
}
#[cfg(test)]
mod tests {
    #[test]
    fn to_sc_handles_pascal_case() {
        assert_eq!(
            super::to_sc(super::ToScInput("HelloWorld")).0,
            "hello_world"
        );
    }
    #[test]
    fn to_sc_collapses_non_alpha_chunks() {
        assert_eq!(super::to_sc(super::ToScInput("A--B__C")).0, "a_b_c");
    }
    #[test]
    fn to_sc_trims_edge_separators() {
        assert_eq!(super::to_sc(super::ToScInput("__Hello__")).0, "hello");
    }
}
