const SC_STRING_MAX_LEN: usize = 1_048_576;
#[derive(optml::Optml, Clone, Copy, newtype::FromInner)]
struct ToSnakeCaseInput<'input_lt>(&'input_lt str);
#[derive(optml::Optml, newtype::BoundedString)]
#[bounded_string(max = SC_STRING_MAX_LEN, description = "snake case string")]
struct SnakeCaseString(String);
#[allow(clippy::single_call_fn)] // extracted to isolate case-normalization logic and keep macro expansion flow focused
fn to_snake_case(input: ToSnakeCaseInput<'_>) -> SnakeCaseString {
    let (normalized, _) = input.0.chars().fold(
        (String::with_capacity(input.0.len()), false),
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
    SnakeCaseString::try_from(naming_common::AsRefStrToSnakeCaseStr::case(&normalized))
        .unwrap_or_else(SnakeCaseString::from)
}
#[proc_macro]
pub fn generate_derive_token_stream_builder(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    #[derive(Clone, optml::Optml)]
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
        .expect("c5d09740")
        .into_iter()
        .map(|element| {
            let sc = to_snake_case(ToSnakeCaseInput::from(element.as_str()));
            Element {
                d_trait_name_upper_camel_case: {
                    let v = naming::parameter::DSelfUpperCamelCase::from_display(&sc.0);
                    quote::quote! {#v}
                },
                d_trait_name_snake_case: {
                    let v = naming::parameter::DSelfSnakeCase::from_display(&sc.0);
                    quote::quote! {#v}
                },
                d_trait_name_if_snake_case: {
                    let v = naming::parameter::DSelfIfSnakeCase::from_display(&sc.0);
                    quote::quote! {#v}
                },
                trait_type: element
                    .parse::<proc_macro2::TokenStream>()
                    .expect("8672240f"),
            }
        })
        .collect::<Vec<Element>>();
    let (make_pub_pub_enum_token_stream, pub_enum_derive_vec_token_stream) = {
        fn generate_token_stream(identifier: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
            quote::quote! {
                #[derive(Debug, Clone, Copy, optml::Optml)]
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
    let generated: proc_macro2::TokenStream = quote::quote! {
        #make_pub_pub_enum_token_stream
        #(#pub_enum_derive_vec_token_stream)*
        #[derive(Debug, Clone, Copy, optml::Optml)]
        enum #struct_or_enum_upper_camel_case {
            Struct,
            Enum
        }
        #[derive(Debug, Default, Clone, Copy, optml::Optml)]
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
            fn build_handle(
                self,
                struct_or_enum: #struct_or_enum_upper_camel_case,
                ann: &dyn #quote_to_tokens_token_stream,
                ident_d8cbb733: &dyn #quote_to_tokens_token_stream,
                generics_7d48c97a: &dyn #quote_to_tokens_token_stream,
                ts: &dyn #quote_to_tokens_token_stream,
            ) -> #ts2_token_stream {
                let maybe_pub_token_stream = self.#make_pub_snake_case_token_stream.then(|| quote::quote!{pub});
                let derive_token_stream = {
                    let mut accumulator_2a71375c = Vec::new();
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
                self.build_handle(
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
                self.build_handle(
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
    fn to_snake_case_handles_pascal_case() {
        assert_eq!(
            super::to_snake_case(super::ToSnakeCaseInput("HelloWorld")).0,
            "hello_world"
        );
    }
    #[test]
    fn to_snake_case_collapses_non_alpha_chunks() {
        assert_eq!(
            super::to_snake_case(super::ToSnakeCaseInput("A--B__C")).0,
            "a_b_c"
        );
    }
    #[test]
    fn to_snake_case_trims_edge_separators() {
        assert_eq!(
            super::to_snake_case(super::ToSnakeCaseInput("__Hello__")).0,
            "hello"
        );
    }
}
