use proc_macro::{Delimiter, TokenStream, TokenTree};
use proc_macro2::TokenStream as Ts2;
use quote::{format_ident, quote};
fn compile_error(msg: &str) -> TokenStream {
    let compile_msg = msg.to_owned();
    quote! {compile_error!(#compile_msg);}.into()
}
fn split_top_level_commas(input: TokenStream) -> Vec<TokenStream> {
    let mut parts = Vec::new();
    let mut current = TokenStream::new();
    let mut angle_depth = 0usize;
    for token in input {
        match &token {
            TokenTree::Punct(punct) if punct.as_char() == '<' => {
                angle_depth = angle_depth.saturating_add(1);
                current.extend([token]);
            }
            TokenTree::Punct(punct) if punct.as_char() == '>' && angle_depth != 0 => {
                angle_depth = angle_depth.saturating_sub(1);
                current.extend([token]);
            }
            TokenTree::Punct(punct) if punct.as_char() == ',' && angle_depth == 0 => {
                parts.push(current);
                current = TokenStream::new();
            }
            TokenTree::Group(_)
            | TokenTree::Ident(_)
            | TokenTree::Punct(_)
            | TokenTree::Literal(_) => {
                current.extend([token]);
            }
        }
    }
    parts.push(current);
    parts
}
fn first_ident(input: &mut impl Iterator<Item = TokenTree>) -> Option<String> {
    match input.next()? {
        TokenTree::Ident(ident) => Some(ident.to_string()),
        TokenTree::Group(group) if group.delimiter() == Delimiter::None => {
            first_ident(&mut group.stream().into_iter())
        }
        TokenTree::Group(_) | TokenTree::Punct(_) | TokenTree::Literal(_) => None,
    }
}
fn strip_first_comma(input: &mut impl Iterator<Item = TokenTree>) -> bool {
    matches!(input.next(), Some(TokenTree::Punct(punct)) if punct.as_char() == ',')
}
fn part_at(parts: &[TokenStream], idx: usize) -> Option<TokenStream> {
    parts.get(idx).cloned()
}
fn first_ident_at(parts: &[TokenStream], idx: usize) -> Option<String> {
    first_ident(&mut part_at(parts, idx)?.into_iter())
}
fn split_fat_arrow(input: TokenStream) -> Option<(TokenStream, TokenStream)> {
    let mut before = TokenStream::new();
    let mut after = TokenStream::new();
    let mut iter = input.into_iter().peekable();
    while let Some(token) = iter.next() {
        if let TokenTree::Punct(punct) = &token
            && punct.as_char() == '='
            && let Some(TokenTree::Punct(next_punct)) = iter.peek()
            && next_punct.as_char() == '>'
        {
            drop(iter.next());
            after.extend(iter);
            return Some((before, after));
        }
        before.extend([token]);
    }
    None
}
#[allow(clippy::single_call_fn)] // this keeps the closure parser isolated from the proc-macro expansion body
fn closure_ident_and_body(input: TokenStream) -> Option<(String, Ts2)> {
    let mut iter = input.into_iter();
    let Some(TokenTree::Punct(open_pipe)) = iter.next() else {
        return None;
    };
    if open_pipe.as_char() != '|' {
        return None;
    }
    let Some(TokenTree::Ident(ident)) = iter.next() else {
        return None;
    };
    let Some(TokenTree::Punct(close_pipe)) = iter.next() else {
        return None;
    };
    if close_pipe.as_char() != '|' {
        return None;
    }
    Some((ident.to_string(), Ts2::from(iter.collect::<TokenStream>())))
}
#[proc_macro]
pub fn tp(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let Some(name) = first_ident(&mut iter) else {
        return compile_error("tp expects type name");
    };
    if !strip_first_comma(&mut iter) {
        return compile_error("tp expects comma after type name");
    }
    let body = iter.collect::<TokenStream>();
    let name_ident = format_ident!("{name}");
    let body_ts = Ts2::from(body);
    quote! {
        #[derive(Debug, Clone, Copy, Optml)]
        pub struct #name_ident;
        impl ToTokens for #name_ident {
            fn to_tokens(&self, tokens: &mut Ts2) {
                append_tokens(tokens, quote! {#body_ts});
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn tp_parts(input: TokenStream) -> TokenStream {
    let mut parts = split_top_level_commas(input);
    if parts.len() < 2 {
        return compile_error("tp_parts expects type name and at least one part");
    }
    let mut name_iter = parts.remove(0).into_iter();
    let Some(name) = first_ident(&mut name_iter) else {
        return compile_error("tp_parts expects type name");
    };
    let name_ident = format_ident!("{name}");
    let part_streams = parts.into_iter().map(Ts2::from).collect::<Vec<Ts2>>();
    quote! {
        #[derive(Debug, Clone, Copy, Optml)]
        pub struct #name_ident;
        impl ToTokens for #name_ident {
            fn to_tokens(&self, tokens: &mut Ts2) {
                #(append_tokens(tokens, #part_streams);)*
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn ts_path_fn(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let Some(name) = first_ident(&mut iter) else {
        return compile_error("ts_path_fn expects function name");
    };
    if !strip_first_comma(&mut iter) {
        return compile_error("ts_path_fn expects comma after function name");
    }
    let body = Ts2::from(iter.collect::<TokenStream>());
    let name_ident = format_ident!("{name}");
    quote! {
        fn #name_ident() -> Ts2 {
            quote! {#body}
        }
    }
    .into()
}
#[proc_macro]
pub fn tp_batch(input: TokenStream) -> TokenStream {
    let mut output = Ts2::new();
    for token in input {
        if let TokenTree::Group(group) = token
            && group.delimiter() == Delimiter::Parenthesis
        {
            let expanded = tp(group.stream());
            output.extend(Ts2::from(expanded));
        }
    }
    output.into()
}
#[proc_macro]
pub fn trait_al(input: TokenStream) -> TokenStream {
    let text = input.to_string();
    let Some((name, bounds)) = text.split_once('=') else {
        return compile_error("trait_al expects Name = Bounds");
    };
    let name_ident = format_ident!("{}", name.trim());
    let Ok(bounds_ts) = bounds.parse::<Ts2>() else {
        return compile_error("trait_al failed to parse bounds");
    };
    quote! {
        pub trait #name_ident: #bounds_ts {}
        impl<T: #bounds_ts> #name_ident for T {}
    }
    .into()
}
#[proc_macro]
pub fn bool_enum_to_tokens(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let Some(name) = first_ident(&mut iter) else {
        return compile_error("bool_enum_to_tokens expects enum name");
    };
    if !strip_first_comma(&mut iter) {
        return compile_error("bool_enum_to_tokens expects comma after enum name");
    }
    let rest_text = iter.collect::<TokenStream>().to_string();
    let Some(rest) = rest_text.strip_prefix("false =>") else {
        return compile_error("bool_enum_to_tokens expects false => expr");
    };
    let Some((false_expr, true_part)) = rest.split_once(", true =>") else {
        return compile_error("bool_enum_to_tokens expects true => expr");
    };
    let Ok(false_ts) = false_expr.trim().parse::<Ts2>() else {
        return compile_error("bool_enum_to_tokens failed to parse false expr");
    };
    let Ok(true_ts) = true_part.trim().parse::<Ts2>() else {
        return compile_error("bool_enum_to_tokens failed to parse true expr");
    };
    let name_ident = format_ident!("{name}");
    quote! {
        #[derive(Debug, Clone, Copy, Optml)]
        pub enum #name_ident {
            False,
            True,
        }
        impl ToTokens for #name_ident {
            fn to_tokens(&self, tokens: &mut Ts2) {
                match &self {
                    Self::False => (#false_ts).to_tokens(tokens),
                    Self::True => (#true_ts).to_tokens(tokens),
                }
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn impl_cfg_getter(input: TokenStream) -> TokenStream {
    let parts = split_top_level_commas(input);
    if parts.len() != 3 {
        return compile_error("impl_cfg_getter expects trait, fn, ret_ty");
    }
    let Some(trait_name) = first_ident_at(&parts, 0) else {
        return compile_error("impl_cfg_getter expects trait name");
    };
    let Some(fn_name) = first_ident_at(&parts, 1) else {
        return compile_error("impl_cfg_getter expects fn name");
    };
    let trait_ident = format_ident!("{trait_name}");
    let fn_ident = format_ident!("{fn_name}");
    let Some(ret_ty) = part_at(&parts, 2).map(Ts2::from) else {
        return compile_error("impl_cfg_getter expects return type");
    };
    quote! {
        impl #trait_ident for ServerAppState<'_> {
            fn #fn_ident(&self) -> &#ret_ty {
                self.cfg_ref().#fn_ident()
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn case_trait_pair(input: TokenStream) -> TokenStream {
    let parts = split_top_level_commas(input);
    if parts.len() != 4 {
        return compile_error("case_trait_pair expects str trait, ts trait, bound, closure expr");
    }
    let Some(str_trait) = first_ident_at(&parts, 0) else {
        return compile_error("case_trait_pair expects string trait name");
    };
    let Some(ts_trait) = first_ident_at(&parts, 1) else {
        return compile_error("case_trait_pair expects token trait name");
    };
    let str_trait_ident = format_ident!("{str_trait}");
    let ts_trait_ident = format_ident!("{ts_trait}");
    let Some(bound_ts) = part_at(&parts, 2).map(Ts2::from) else {
        return compile_error("case_trait_pair expects bound");
    };
    let Some(closure_text) = part_at(&parts, 3).map(|part| part.to_string()) else {
        return compile_error("case_trait_pair expects closure");
    };
    let Some((param_part, body_part)) = closure_text
        .split_once('|')
        .and_then(|(_, rest)| rest.split_once('|'))
    else {
        return compile_error("case_trait_pair expects closure");
    };
    let param_ident = format_ident!("{}", param_part.trim());
    let Ok(body_ts) = body_part.trim().parse::<Ts2>() else {
        return compile_error("case_trait_pair failed to parse body");
    };
    quote! {
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
            fn case_or_panic(&self) -> Ts2;
        }
        impl<T> #ts_trait_ident for T
        where
            T: #str_trait_ident,
        {
            fn case_or_panic(&self) -> Ts2 {
                to_ts_or_panic(&#str_trait_ident::case(self))
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn loc(input: TokenStream) -> TokenStream {
    drop(input);
    quote! {
        loc_lib::loc::Loc::new(file!().to_owned(), line!(), column!(), None)
    }
    .into()
}
#[proc_macro]
pub fn impl_to_err_string_with(input: TokenStream) -> TokenStream {
    let Some((types_raw, closure)) = split_fat_arrow(input) else {
        return compile_error("impl_to_err_string_with expects types => |value| body");
    };
    let Some((value, body)) = closure_ident_and_body(closure) else {
        return compile_error("impl_to_err_string_with expects closure");
    };
    let value_ident = format_ident!("{value}");
    let types = split_top_level_commas(types_raw)
        .into_iter()
        .map(Ts2::from)
        .collect::<Vec<Ts2>>();
    quote! {
        #(impl ToErrString for #types {
            fn to_err_string(&self) -> String {
                let #value_ident = self;
                #body
            }
        })*
    }
    .into()
}
#[proc_macro]
pub fn impl_to_err_string_const(input: TokenStream) -> TokenStream {
    let mut types = Vec::new();
    let mut msgs = Vec::new();
    for part in split_top_level_commas(input) {
        if part.is_empty() {
            continue;
        }
        let Some((ty, msg)) = split_fat_arrow(part) else {
            return compile_error("impl_to_err_string_const expects type => msg");
        };
        types.push(Ts2::from(ty));
        msgs.push(Ts2::from(msg));
    }
    quote! {
        #(impl ToErrString for #types {
            fn to_err_string(&self) -> String {
                static_str_to_owned(#msgs)
            }
        })*
    }
    .into()
}
#[proc_macro]
pub fn impl_to_err_string_as_ref_str(input: TokenStream) -> TokenStream {
    let types = split_top_level_commas(input)
        .into_iter()
        .filter(|part| !part.is_empty())
        .map(Ts2::from)
        .collect::<Vec<Ts2>>();
    quote! {
        #(impl ToErrString for #types {
            fn to_err_string(&self) -> String {
                as_ref_str_to_owned(self)
            }
        })*
    }
    .into()
}
#[proc_macro]
pub fn impl_try_from_non_empty_string(input: TokenStream) -> TokenStream {
    let parts = split_top_level_commas(input);
    if parts.len() != 2 {
        return compile_error("impl_try_from_non_empty_string expects name, error name");
    }
    let Some(name_text) = first_ident_at(&parts, 0) else {
        return compile_error("impl_try_from_non_empty_string expects name");
    };
    let Some(er_name_text) = first_ident_at(&parts, 1) else {
        return compile_error("impl_try_from_non_empty_string expects error name");
    };
    let name = format_ident!("{name_text}");
    let er_name = format_ident!("{er_name_text}");
    quote! {
        #[derive(Debug, Clone, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
        pub struct #name(pub String);
        #[derive(Debug, Clone, Copy, Error, Optml)]
        pub enum #er_name {
            #[error("{is_empty:?}")]
            IsEmpty { is_empty: &'static str },
        }
        impl TryFromStdEnvVarOk for #name {
            type Error = #er_name;
            fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
                try_map_non_empty_env_value(v, |is_empty| Self::Error::IsEmpty { is_empty }, Self)
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn impl_try_from_secret_url(input: TokenStream) -> TokenStream {
    let parts = split_top_level_commas(input);
    if parts.len() != 2 {
        return compile_error("impl_try_from_secret_url expects name, error name");
    }
    let Some(name_text) = first_ident_at(&parts, 0) else {
        return compile_error("impl_try_from_secret_url expects name");
    };
    let Some(er_name_text) = first_ident_at(&parts, 1) else {
        return compile_error("impl_try_from_secret_url expects error name");
    };
    let name = format_ident!("{name_text}");
    let er_name = format_ident!("{er_name_text}");
    quote! {
        #[derive(Debug, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
        pub struct #name(pub SecretBox<String>);
        #[derive(Debug, Clone, Copy, Error, Optml)]
        pub enum #er_name {
            #[error("{is_empty:?}")]
            IsEmpty { is_empty: &'static str },
        }
        impl TryFromStdEnvVarOk for #name {
            type Error = #er_name;
            fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
                try_map_non_empty_env_value(
                    v,
                    |is_empty| Self::Error::IsEmpty { is_empty },
                    |v| Self(SecretBox::new(Box::new(v))),
                )
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn impl_try_from_parse(input: TokenStream) -> TokenStream {
    impl_try_from_parse_with_er_ty(input, None)
}
#[proc_macro]
pub fn impl_try_from_parse_string_er(input: TokenStream) -> TokenStream {
    impl_try_from_parse_with_er_ty(input, Some(quote! {String}))
}
fn impl_try_from_parse_with_er_ty(input: TokenStream, fixed_er_ty: Option<Ts2>) -> TokenStream {
    let parts = split_top_level_commas(input);
    let min_len = if fixed_er_ty.is_some() { 5 } else { 6 };
    if parts.len() < min_len {
        return compile_error(
            "impl_try_from_parse expects name, error name, inner type and error variant",
        );
    }
    let Some(name_text) = first_ident_at(&parts, 0) else {
        return compile_error("impl_try_from_parse expects name");
    };
    let Some(er_name_text) = first_ident_at(&parts, 1) else {
        return compile_error("impl_try_from_parse expects error name");
    };
    let Some(er_vrt_text) = first_ident_at(&parts, 3) else {
        return compile_error("impl_try_from_parse expects error variant");
    };
    let Some(er_field_text) = first_ident_at(&parts, 4) else {
        return compile_error("impl_try_from_parse expects error field");
    };
    let name = format_ident!("{name_text}");
    let er_name = format_ident!("{er_name_text}");
    let er_vrt = format_ident!("{er_vrt_text}");
    let er_field = format_ident!("{er_field_text}");
    let Some(inner) = part_at(&parts, 2).map(Ts2::from) else {
        return compile_error("impl_try_from_parse expects inner type");
    };
    let (er_ty, derives) = fixed_er_ty.map_or_else(
        || {
            let Some(er_ty) = part_at(&parts, 5).map(Ts2::from) else {
                return (Ts2::new(), Vec::new());
            };
            let derives = parts
                .get(6..)
                .unwrap_or(&[])
                .iter()
                .cloned()
                .map(Ts2::from)
                .collect::<Vec<Ts2>>();
            (er_ty, derives)
        },
        |fixed_er_ty_value| (fixed_er_ty_value, vec![quote! {Clone}, quote! {Copy}]),
    );
    quote! {
        #[derive(Debug, #(#derives,)* gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
        pub struct #name(pub #inner);
        #[derive(Debug, Error, Optml)]
        pub enum #er_name {
            #[error("{:?}", .#er_field)]
            #er_vrt { #er_field: #er_ty },
        }
        impl TryFromStdEnvVarOk for #name {
            type Error = #er_name;
            fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
                parse_from_str_with_er(&v, |#er_field| Self::Error::#er_vrt { #er_field }).map(Self)
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn assert_parse_ok_matches(input: TokenStream) -> TokenStream {
    let parts = split_top_level_commas(input);
    if parts.len() != 3 {
        return compile_error("assert_parse_ok_matches expects type, value, pattern");
    }
    let Some(ty) = part_at(&parts, 0).map(Ts2::from) else {
        return compile_error("assert_parse_ok_matches expects type");
    };
    let Some(value) = part_at(&parts, 1).map(Ts2::from) else {
        return compile_error("assert_parse_ok_matches expects value");
    };
    let Some(pattern) = part_at(&parts, 2).map(Ts2::from) else {
        return compile_error("assert_parse_ok_matches expects pattern");
    };
    quote! {
        assert!(matches!(parse_env::<#ty>(#value), Ok(#pattern)));
    }
    .into()
}
#[proc_macro]
pub fn assert_parse_err_matches(input: TokenStream) -> TokenStream {
    let parts = split_top_level_commas(input);
    if parts.len() != 3 {
        return compile_error("assert_parse_err_matches expects type, value, pattern");
    }
    let Some(ty) = part_at(&parts, 0).map(Ts2::from) else {
        return compile_error("assert_parse_err_matches expects type");
    };
    let Some(value) = part_at(&parts, 1).map(Ts2::from) else {
        return compile_error("assert_parse_err_matches expects value");
    };
    let Some(pattern) = part_at(&parts, 2).map(Ts2::from) else {
        return compile_error("assert_parse_err_matches expects pattern");
    };
    quote! {
        assert!(matches!(parse_env::<#ty>(#value), Err(#pattern)));
    }
    .into()
}
#[proc_macro]
pub fn assert_empty_parse_err_matches(input: TokenStream) -> TokenStream {
    let parts = split_top_level_commas(input);
    if parts.len() != 2 {
        return compile_error("assert_empty_parse_err_matches expects type, pattern");
    }
    let Some(ty) = part_at(&parts, 0).map(Ts2::from) else {
        return compile_error("assert_empty_parse_err_matches expects type");
    };
    let Some(pattern) = part_at(&parts, 1).map(Ts2::from) else {
        return compile_error("assert_empty_parse_err_matches expects pattern");
    };
    quote! {
        assert!(matches!(parse_env::<#ty>(""), Err(#pattern)));
    }
    .into()
}
