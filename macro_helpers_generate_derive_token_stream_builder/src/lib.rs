mod domain_types;

// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::single_call_fn)] // extracted to isolate case-normalization logic and keep macro expansion flow focused
fn to_snake_case(input: domain_types::ToSnakeCaseInput<'_>) -> domain_types::SnakeCaseString {
    let (normalized, _) = input.as_ref().chars().fold(
        (String::with_capacity(input.as_ref().len()), false),
        |(mut normalized, separator_pending), ch| {
            if char::is_alphanumeric(ch) {
                if separator_pending && !normalized.is_empty() {
                    normalized.push(' ');
                }
                normalized.push(ch);
                (normalized, false)
            } else {
                let next_separator_pending = !normalized.is_empty();
                (normalized, next_separator_pending)
            }
        },
    );
    domain_types::SnakeCaseString::try_from(
        naming_common::domain_types::AsRefStrToSnakeCaseStr::case(&normalized),
    )
    .unwrap_or_else(domain_types::SnakeCaseString::from)
}
#[proc_macro]
pub fn generate_derive_token_stream_builder(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    #[derive(Clone, optimal_memory_layout::OptimalMemoryLayout)]
    struct Element {
        d_trait_name_if_snake_case: proc_macro2::TokenStream,
        d_trait_name_snake_case: proc_macro2::TokenStream,
        d_trait_name_upper_camel_case: proc_macro2::TokenStream,
        trait_type: proc_macro2::TokenStream,
    }
    let make_pub_snake_case_token_stream = quote::quote! {make_pub};
    let make_pub_if_snake_case_token_stream = quote::quote! {make_pub_if};
    let make_pub_upper_camel_case_token_stream = quote::quote! {MakePub};
    let element_vec = serde_json::from_str::<Vec<String>>(&input_token_stream.to_string())
        .expect("c5d09740 generate_derive_token_stream_builder invariant must hold")
        .into_iter()
        .map(|element| {
            let sc = to_snake_case(domain_types::ToSnakeCaseInput::from(element.as_str()));
            Element {
                d_trait_name_upper_camel_case: {
                    let v = naming::domain_types::parameter::DSelfUpperCamelCase::from_display(
                        &sc.as_ref(),
                    );
                    quote::quote! {#v}
                },
                d_trait_name_snake_case: {
                    let v =
                        naming::domain_types::parameter::DSelfSnakeCase::from_display(&sc.as_ref());
                    quote::quote! {#v}
                },
                d_trait_name_if_snake_case: {
                    let v = naming::domain_types::parameter::DSelfIfSnakeCase::from_display(
                        &sc.as_ref(),
                    );
                    quote::quote! {#v}
                },
                trait_type: element
                    .parse::<proc_macro2::TokenStream>()
                    .expect("8672240f generate_derive_token_stream_builder invariant must hold"),
            }
        })
        .collect::<Vec<Element>>();
    let (make_pub_pub_enum_token_stream, pub_enum_derive_vec_token_stream) = {
        fn generate_token_stream(identifier: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
            quote::quote! {
                #[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
                pub enum #identifier {
                    True,
                    False
                }
            }
        }
        (
            generate_token_stream(&make_pub_upper_camel_case_token_stream),
            element_vec
                .iter()
                .map(|element| generate_token_stream(&element.d_trait_name_upper_camel_case)),
        )
    };
    let (make_pub_derive_trait_name_bool_token_stream, field_vec_token_stream) = {
        fn generate_token_stream(identifier: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
            quote::quote! {#identifier: bool,}
        }
        (
            generate_token_stream(&make_pub_snake_case_token_stream),
            element_vec
                .iter()
                .map(|element| generate_token_stream(&element.d_trait_name_snake_case)),
        )
    };
    let (make_pub_derive_and_derive_if_token_stream, derive_and_derive_if_vec_token_stream) = {
        let generate_token_stream =
            |first_name_token_stream: &dyn quote::ToTokens,
             second_name_token_stream: &dyn quote::ToTokens,
             condition_type_token_stream: &dyn quote::ToTokens| {
                quote::quote! {
                    pub const fn #first_name_token_stream(mut self) -> Self {
                        self.#first_name_token_stream = true;
                        self
                    }
                    pub const fn #second_name_token_stream(mut self, condition: #condition_type_token_stream) -> Self {
                        if let #condition_type_token_stream::True = condition {
                            self.#first_name_token_stream = true;
                        }
                        self
                    }
                }
            };
        (
            generate_token_stream(
                &make_pub_snake_case_token_stream,
                &make_pub_if_snake_case_token_stream,
                &make_pub_upper_camel_case_token_stream,
            ),
            {
                let ts = element_vec.iter().map(|element| {
                    generate_token_stream(
                        &element.d_trait_name_snake_case,
                        &element.d_trait_name_if_snake_case,
                        &element.d_trait_name_upper_camel_case,
                    )
                });
                quote::quote! {#(#ts)*}
            },
        )
    };
    let if_self_derive_accumulator_push_vec_token_stream = element_vec.iter().map(|element| {
        let d_trait_name_snake_case = &element.d_trait_name_snake_case;
        let trait_type = &element.trait_type;
        quote::quote! {
            if self.#d_trait_name_snake_case {
                accumulator_2a71375c.push(quote::quote!{#trait_type});
            }
        }
    });
    let derive_token_stream_builder_upper_camel_case = quote::quote! {DTokenStreamBuilder};
    let struct_or_enum_upper_camel_case = quote::quote! {StructOrEnum};
    let quote_to_tokens_token_stream = quote::quote! {quote::ToTokens};
    let ts2_token_stream = quote::quote! {proc_macro2::TokenStream};
    let element_count = element_vec.len();
    let generated: proc_macro2::TokenStream = quote::quote! {
        #make_pub_pub_enum_token_stream
        #(#pub_enum_derive_vec_token_stream)*
        #[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
        enum #struct_or_enum_upper_camel_case {
            Struct,
            Enum
        }
        #[derive(Debug, Default, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
        pub struct #derive_token_stream_builder_upper_camel_case {
            #make_pub_derive_trait_name_bool_token_stream
            #(#field_vec_token_stream)*
        }
        impl #derive_token_stream_builder_upper_camel_case {
            pub fn new() -> Self {
                Self::default()
            }
            #make_pub_derive_and_derive_if_token_stream
            #derive_and_derive_if_vec_token_stream
            fn build_declaration(
                self,
                struct_or_enum: #struct_or_enum_upper_camel_case,
                ann: &dyn #quote_to_tokens_token_stream,
                ident_d8cbb733: &dyn #quote_to_tokens_token_stream,
                generics_7d48c97a: &dyn #quote_to_tokens_token_stream,
                ts: &dyn #quote_to_tokens_token_stream,
            ) -> #ts2_token_stream {
                let maybe_pub_token_stream = self.#make_pub_snake_case_token_stream.then(|| quote::quote!{pub});
                let derive_token_stream = {
                    let mut accumulator_2a71375c = Vec::with_capacity(#element_count);
                    #(#if_self_derive_accumulator_push_vec_token_stream)*
                    accumulator_2a71375c
                };
                let struct_or_enum_token_stream = match struct_or_enum {
                    #struct_or_enum_upper_camel_case::Struct => quote::quote!{struct},
                    #struct_or_enum_upper_camel_case::Enum => quote::quote!{enum},
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
                                        // The owner module retains lint-sensitive semantics from the original implementation.
                                        #[allow(unused_mut)]
                                        let (mut derive_token_stream, i) = derive_token_stream
                                            .quote_into_iter();
                                        let has_iter = has_iter | i;
                                        <_ as ::quote::__private::CheckHasIterator<
                                            true,
                                        >>::check(has_iter);
                                        loop {
                                            let derive_token_stream = match derive_token_stream.next() {
                                                Some(_x) => ::quote::__private::RepInterp(_x),
                                                None => break,
                                            };
                                            if _i > 0 {
                                                ::quote::__private::push_comma(&mut _s);
                                            }
                                            _i += 1;
                                            ::#quote_to_tokens_token_stream::to_tokens(&derive_token_stream, &mut _s);
                                        }
                                    }
                                    _s
                                },
                            );
                            _s
                        },
                    );
                    ::#quote_to_tokens_token_stream::to_tokens(&ann, &mut _s);
                    ::#quote_to_tokens_token_stream::to_tokens(&maybe_pub_token_stream, &mut _s);
                    ::#quote_to_tokens_token_stream::to_tokens(&struct_or_enum_token_stream, &mut _s);
                    ::#quote_to_tokens_token_stream::to_tokens(&ident_d8cbb733, &mut _s);
                    ::#quote_to_tokens_token_stream::to_tokens(&generics_7d48c97a, &mut _s);
                    ::#quote_to_tokens_token_stream::to_tokens(&ts, &mut _s);
                    _s
                }
            }
            pub fn build_struct(
                self,
                ann: &dyn #quote_to_tokens_token_stream,
                ident_d87c6809: &dyn #quote_to_tokens_token_stream,
                generics_c33a0ef2: &dyn #quote_to_tokens_token_stream,
                ts: &dyn #quote_to_tokens_token_stream,
            ) -> #ts2_token_stream {
                self.build_declaration(
                    #struct_or_enum_upper_camel_case::Struct,
                    ann,
                    ident_d87c6809,
                    generics_c33a0ef2,
                    ts
                )
            }
            pub fn build_enum(
                self,
                ann: &dyn #quote_to_tokens_token_stream,
                ident_273dd063: &dyn #quote_to_tokens_token_stream,
                generics_84bc3f7f: &dyn #quote_to_tokens_token_stream,
                ts: &dyn #quote_to_tokens_token_stream,
            ) -> #ts2_token_stream {
                self.build_declaration(
                    #struct_or_enum_upper_camel_case::Enum,
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
    fn to_snake_case_converts_pascal_case() {
        assert_eq!(
            super::to_snake_case(super::domain_types::ToSnakeCaseInput::from("HelloWorld"))
                .as_ref(),
            "hello_world"
        );
    }
    #[test]
    fn to_snake_case_collapses_non_alpha_chunks() {
        assert_eq!(
            super::to_snake_case(super::domain_types::ToSnakeCaseInput::from("A--B__C")).as_ref(),
            "a_b_c"
        );
    }
    #[test]
    fn to_snake_case_trims_edge_separators() {
        assert_eq!(
            super::to_snake_case(super::domain_types::ToSnakeCaseInput::from("__Hello__")).as_ref(),
            "hello"
        );
    }
}
