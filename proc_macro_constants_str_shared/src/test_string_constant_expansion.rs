#[test]
fn test_string_constant_expansion_rejects_duplicate_names_and_unknown_fragments() {
    let word = syn::LitStr::new(stringify!(word), proc_macro2::Span::call_site());
    assert!([
        (quote::quote! { fragments { Word = #word; Word = #word; } rust_fragments {} rust_constants {} constants {} }, stringify!(5bbbde57)),
        (quote::quote! { fragments {} rust_fragments {} rust_constants {} constants { First = []; First = []; } }, stringify!(ad857256)),
        (quote::quote! { fragments {} rust_fragments {} rust_constants {} constants { First = [Missing]; } }, stringify!(bb09ab55)),
        (quote::quote! { fragments {} rust_fragments {} rust_constants { First = []; } constants { First = []; } }, stringify!(ad857256)),
        (quote::quote! { fragments { Word = #word; } rust_fragments { Word = ["("]; } rust_constants {} constants {} }, stringify!(750ff794)),
    ].into_iter().all(|(input, expected)| crate::define_str_constants(input).to_string().contains(expected)));
}

#[test]
fn test_string_constant_expansion_validates_word_fragment_values_and_reuse() {
    let word = syn::LitStr::new(stringify!(word), proc_macro2::Span::call_site());
    assert!(
        [
            (
                quote::quote! { fragments { Word = ""; } },
                stringify!(3bc0da90)
            ),
            (
                quote::quote! { fragments { Word = " "; } },
                stringify!(3bc0da90)
            ),
            (
                quote::quote! { fragments { Word = #word; Other = #word; } },
                stringify!(0566e947)
            ),
            (
                quote::quote! { fragments { Word = #word; } },
                stringify!(34090e38)
            ),
        ]
        .into_iter()
        .all(|(fragments, expected)| crate::define_str_constants(
            quote::quote! { #fragments rust_fragments {} rust_constants {} constants {} }
        )
        .to_string()
        .contains(expected))
    );
}

#[test]
fn test_string_constant_expansion_reuses_equal_values_and_preserves_visibility() {
    let word = syn::LitStr::new(stringify!(word), proc_macro2::Span::call_site());
    let output = crate::define_str_constants(quote::quote! {
        fragments { Word = #word; } rust_fragments {} rust_constants {}
        constants { pub First = [Word]; pub(crate) Second = [Word]; }
    });
    assert_eq!(
        output.to_string(),
        quote::quote! { pub const First: &str = #word; pub(crate) const Second: &str = First; }
            .to_string()
    );
}

#[test]
fn test_git_info_generator_rejects_input_and_emits_both_commit_constants() {
    assert!(
        crate::define_git_info_constants(&quote::quote!(Input))
            .to_string()
            .contains(stringify!(78de8960))
    );
    assert!(
        syn::parse2::<syn::File>(crate::define_git_info_constants(&quote::quote!())).is_ok_and(
            |file| {
                matches!(file.items.as_slice(), [syn::Item::Const(commit), syn::Item::Const(link)]
            if commit.ident == stringify!(GIT_INFO_PROJECT_GIT_COMMIT_ID)
                && link.ident == stringify!(GIT_INFO_PROJECT_GIT_COMMIT_LINK))
            }
        )
    );
}

#[test]
fn test_string_constant_expansion_validates_rust_fragment_definitions() {
    let word = syn::LitStr::new(stringify!(word), proc_macro2::Span::call_site());
    assert!(
        [
            (quote::quote! { Rust = []; }, stringify!(f9805250)),
            (quote::quote! { Rust = [Missing]; }, stringify!(38b81d16)),
            (
                quote::quote! { Rust = ["("]; Other = [Rust]; },
                stringify!(38b81d16)
            ),
            (quote::quote! { Rust = [#word]; }, stringify!(9cf0b14e)),
            (
                quote::quote! { Rust = ["("]; Rust = [")"]; },
                stringify!(750ff794)
            ),
            (quote::quote! { Rust = ["("]; }, stringify!(485d9907)),
        ]
        .into_iter()
        .all(
            |(fragments, expected)| crate::define_str_constants(quote::quote! {
                fragments {} rust_fragments { #fragments } rust_constants {} constants {}
            })
            .to_string()
            .contains(expected)
        )
    );
}

#[test]
fn test_string_constant_expansion_composes_reused_rust_syntax_fragments() {
    let syntax = syn::LitStr::new("()", proc_macro2::Span::call_site());
    assert_eq!(
        crate::define_str_constants(quote::quote! {
            fragments {} rust_fragments { Syntax = [#syntax]; }
            rust_constants { First = [Syntax]; Second = [Syntax]; } constants {}
        })
        .to_string(),
        quote::quote! { const First: &str = #syntax; const Second: &str = First; }.to_string()
    );
}

#[test]
fn test_string_constant_expansion_rejects_literal_bypasses_of_fragments() {
    let word = syn::LitStr::new(stringify!(word), proc_macro2::Span::call_site());
    let syntax = syn::LitStr::new("()", proc_macro2::Span::call_site());
    assert!([
        (quote::quote! { fragments {} rust_fragments {} rust_constants {} constants { First = [#word]; Second = [#word]; } }, stringify!(5515a1e9)),
        (quote::quote! { fragments { Word = #word; } rust_fragments {} rust_constants {} constants { First = [Word, Word]; Second = [#word]; } }, stringify!(fe0fa60a)),
        (quote::quote! { fragments {} rust_fragments {} rust_constants {} constants { First = [#syntax]; Second = [#syntax]; } }, stringify!(f37cb2a6)),
        (quote::quote! { fragments {} rust_fragments { Syntax = [#syntax]; } rust_constants { First = [Syntax, Syntax]; } constants { Second = [#syntax]; } }, stringify!(c84d79e1)),
    ].into_iter().all(|(input, expected)| crate::define_str_constants(input).to_string().contains(expected)));
}

#[test]
fn test_string_constant_expansion_rejects_invalid_block_structure() {
    assert!([
        quote::quote!(),
        quote::quote! { constants {} },
        quote::quote! { fragments {} rust_fragments {} rust_constants {} constants { First = []; } Extra },
        quote::quote! { fragments {} rust_fragments {} rust_constants {} constants { First = [] } },
    ].into_iter().all(|input| crate::define_str_constants(input).to_string().contains(stringify!(compile_error))));
}

#[test]
fn test_string_constant_expansion_counts_word_reuse_inside_rust_fragments() {
    let word = syn::LitStr::new(stringify!(word), proc_macro2::Span::call_site());
    let expected = syn::LitStr::new(
        format!("{}()", word.value()).as_str(),
        proc_macro2::Span::call_site(),
    );
    assert_eq!(
        crate::define_str_constants(quote::quote! {
            fragments { Word = #word; }
            rust_fragments { Syntax = [Word, "()"]; }
            rust_constants { First = [Syntax]; Second = [Syntax]; }
            constants { Third = [Word]; }
        })
        .to_string(),
        quote::quote! {
            const First: &str = #expected;
            const Second: &str = First;
            const Third: &str = #word;
        }
        .to_string()
    );
}

#[test]
fn test_string_constant_parser_enforces_each_collection_limit() {
    let word = syn::LitStr::new(stringify!(word), proc_macro2::Span::call_site());
    assert!([crate::COLLECTION_MAX_LEN, crate::COLLECTION_MAX_LEN + 1usize].into_iter().all(|count| {
        let fragments = std::iter::repeat_n(quote::quote!(Word = #word;), count);
        let rust_fragments = std::iter::repeat_n(quote::quote!(Syntax = [];), count);
        let constants = std::iter::repeat_n(quote::quote!(First = [];), count);
        let parts = std::iter::repeat_n(quote::quote!(Word), count);
        [
            (quote::quote! { fragments { #(#fragments)* } rust_fragments {} rust_constants {} constants {} }, "883ea6b2"),
            (quote::quote! { fragments {} rust_fragments { #(#rust_fragments)* } rust_constants {} constants {} }, stringify!(c31f6dd7)),
            (quote::quote! { fragments {} rust_fragments {} rust_constants {} constants { #(#constants)* } }, stringify!(2bd1b963)),
            (quote::quote! { fragments {} rust_fragments {} rust_constants {} constants { First = [#(#parts),*]; } }, stringify!(c93f714a)),
        ].into_iter().all(|(input, expected)| {
            let result = syn::parse2::<crate::DefineStrConstantsInput>(input);
            if count == crate::COLLECTION_MAX_LEN {
                result.is_ok_and(|parsed| parsed.constants.0.len() <= crate::COLLECTION_MAX_LEN && parsed.fragments.0.len() <= crate::COLLECTION_MAX_LEN && parsed.rust_fragments.0.len() <= crate::COLLECTION_MAX_LEN)
            } else {
                result.is_err_and(|error| error.to_string().contains(expected))
            }
        })
    }));
}
