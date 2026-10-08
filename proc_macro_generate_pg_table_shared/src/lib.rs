#![allow(
    clippy::must_use_candidate,
    clippy::useless_conversion,
    reason = "shared proc-macro implementations preserve original entrypoint conversion points while returning proc_macro2 streams to one-entrypoint facade crates; every result is consumed immediately by its facade"
)]

pub fn generate_pg_table_config(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn create_many_error_variants(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn read_many_error_variants(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn update_many_error_variants(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn delete_many_error_variants(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn common_error_variants(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn create_many_logic(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn read_many_logic(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn update_many_logic(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn delete_many_logic(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn common_logic(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn derive_generate_pg_table(
    token_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let input_token_stream = token_stream.into();
    generate_pg_table_src::generate_pg_table::generate_pg_table(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
            &input_token_stream,
        ),
    )
    .to_string()
    .parse::<proc_macro2::TokenStream>()
    .expect(constants_str::DIAGNOSTIC_6BFF799B)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_table_annotation_adapters_preserve_items_and_ignore_attributes() {
        let token_stream = || {
            proc_macro2::TokenStream::from(proc_macro2::TokenTree::Ident(proc_macro2::Ident::new(
                constants_str::X,
                proc_macro2::Span::call_site(),
            )))
        };
        (0u8..11u8).fold((), |(), adapter| {
            [proc_macro2::TokenStream::new(), token_stream()]
                .into_iter()
                .fold((), |(), attributes| {
                    [
                        proc_macro2::TokenStream::new(),
                        token_stream(),
                        proc_macro2::TokenStream::from(proc_macro2::TokenTree::Group(
                            proc_macro2::Group::new(proc_macro2::Delimiter::Brace, token_stream()),
                        )),
                    ]
                    .into_iter()
                    .fold((), |(), item| {
                        let output = match adapter {
                            0u8 => {
                                crate::generate_pg_table_config(attributes.clone(), item.clone())
                            }
                            1u8 => {
                                crate::create_many_error_variants(attributes.clone(), item.clone())
                            }
                            2u8 => {
                                crate::read_many_error_variants(attributes.clone(), item.clone())
                            }
                            3u8 => {
                                crate::update_many_error_variants(attributes.clone(), item.clone())
                            }
                            4u8 => {
                                crate::delete_many_error_variants(attributes.clone(), item.clone())
                            }
                            5u8 => crate::common_error_variants(attributes.clone(), item.clone()),
                            6u8 => crate::create_many_logic(attributes.clone(), item.clone()),
                            7u8 => crate::read_many_logic(attributes.clone(), item.clone()),
                            8u8 => crate::update_many_logic(attributes.clone(), item.clone()),
                            9u8 => crate::delete_many_logic(attributes.clone(), item.clone()),
                            _ => crate::common_logic(attributes.clone(), item.clone()),
                        };
                        assert_eq!(output.to_string(), item.to_string());
                    });
                });
        });
    }
}
