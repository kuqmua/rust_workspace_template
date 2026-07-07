#[proc_macro]
pub fn case_trait_pair(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas(input.into());
    if parts.len() != 4 {
        return workspace_macro_helpers::compile_error_ts(
            "case_trait_pair expects str trait, ts trait, bound, closure expr",
        )
        .into();
    }
    let Some(str_trait) = workspace_macro_helpers::first_ident_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_ts(
            "case_trait_pair expects string trait name",
        )
        .into();
    };
    let Some(ts_trait) = workspace_macro_helpers::first_ident_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_ts(
            "case_trait_pair expects token trait name",
        )
        .into();
    };
    let str_trait_ident = quote::format_ident!("{str_trait}");
    let ts_trait_ident = quote::format_ident!("{ts_trait}");
    let Some(bound_ts) = workspace_macro_helpers::part_at(&parts, 2) else {
        return workspace_macro_helpers::compile_error_ts("case_trait_pair expects bound").into();
    };
    let Some(closure_text) =
        workspace_macro_helpers::part_at(&parts, 3).map(|part| part.to_string())
    else {
        return workspace_macro_helpers::compile_error_ts("case_trait_pair expects closure").into();
    };
    let Some((param_part, body_part)) = closure_text
        .split_once('|')
        .and_then(|(_, rest)| rest.split_once('|'))
    else {
        return workspace_macro_helpers::compile_error_ts("case_trait_pair expects closure").into();
    };
    let param_ident = quote::format_ident!("{}", param_part.trim());
    let Ok(body_ts) = body_part.trim().parse::<proc_macro2::TokenStream>() else {
        return workspace_macro_helpers::compile_error_ts("case_trait_pair failed to parse body")
            .into();
    };
    quote::quote! {
        pub trait #str_trait_ident {
            fn case(&self) -> String;
        }
        impl<T> #str_trait_ident for T
        where
            T: #bound_ts,
        {
            fn case(&self) -> String {
                let #param_ident = self;
                #body_ts
            }
        }
        pub trait #ts_trait_ident {
            fn case_or_panic(&self) -> proc_macro2::TokenStream;
        }
        impl<T> #ts_trait_ident for T
        where
            T: #str_trait_ident,
        {
            fn case_or_panic(&self) -> proc_macro2::TokenStream {
                to_ts_or_panic(&#str_trait_ident::case(self))
            }
        }
    }
    .into()
}
