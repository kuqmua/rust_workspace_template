fn with_attr_token_stream(
    attr_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    quote::quote! {
        #attr_token_stream
        #ts
    }
    .into()
}
fn const_space_token_stream(
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    quote::quote! {const #ts}.into()
}
fn pub_space_token_stream(
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    quote::quote! {pub #ts}.into()
}
fn impl_identifier_token_stream(
    identifier_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    quote::quote! {
        impl #identifier_token_stream {
            #ts
        }
    }
    .into()
}
pub fn generate_new_token_stream(
    attr_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    quote::quote! {
        #attr_token_stream
        fn new(#parameters_token_stream) -> Self {
            #ts
        }
    }
    .into()
}
pub fn generate_const_new_token_stream(
    attr_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    let ts_5986cf7b = const_space_token_stream(&generate_new_token_stream(
        &proc_macro2::TokenStream::new(),
        parameters_token_stream,
        ts,
    ));
    with_attr_token_stream(attr_token_stream, &ts_5986cf7b)
}
pub fn generate_pub_new_token_stream(
    attr_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    let ts_73940779 = pub_space_token_stream(&generate_new_token_stream(
        &proc_macro2::TokenStream::new(),
        parameters_token_stream,
        ts,
    ));
    with_attr_token_stream(attr_token_stream, &ts_73940779)
}
pub fn generate_pub_const_new_token_stream(
    attr_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    let ts_5dc3668f = pub_space_token_stream(&generate_const_new_token_stream(
        &proc_macro2::TokenStream::new(),
        parameters_token_stream,
        ts,
    ));
    with_attr_token_stream(attr_token_stream, &ts_5dc3668f)
}
pub fn generate_impl_new_for_identifier_token_stream(
    identifier_token_stream: &dyn quote::ToTokens,
    attr_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    impl_identifier_token_stream(
        identifier_token_stream,
        &generate_new_token_stream(attr_token_stream, parameters_token_stream, ts),
    )
}
pub fn generate_impl_const_new_for_identifier_token_stream(
    identifier_token_stream: &dyn quote::ToTokens,
    attr_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    impl_identifier_token_stream(
        identifier_token_stream,
        &generate_const_new_token_stream(attr_token_stream, parameters_token_stream, ts),
    )
}
pub fn generate_impl_pub_new_for_identifier_token_stream(
    identifier_token_stream: &dyn quote::ToTokens,
    attr_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    impl_identifier_token_stream(
        identifier_token_stream,
        &generate_pub_new_token_stream(attr_token_stream, parameters_token_stream, ts),
    )
}
pub fn generate_impl_pub_const_new_for_identifier_token_stream(
    identifier_token_stream: &dyn quote::ToTokens,
    attr_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    impl_identifier_token_stream(
        identifier_token_stream,
        &generate_pub_const_new_token_stream(attr_token_stream, parameters_token_stream, ts),
    )
}
pub fn generate_try_new_token_stream(
    attr_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    quote::quote! {
        #attr_token_stream
        fn try_new(#parameters_token_stream) -> Result<Self, #err_type_token_stream> {
            #ts
        }
    }
    .into()
}
pub fn generate_const_try_new_token_stream(
    attr_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    let ts0 = const_space_token_stream(&generate_try_new_token_stream(
        &proc_macro2::TokenStream::new(),
        parameters_token_stream,
        err_type_token_stream,
        ts,
    ));
    with_attr_token_stream(attr_token_stream, &ts0)
}
pub fn generate_pub_try_new_token_stream(
    attr_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    let ts0 = pub_space_token_stream(&generate_try_new_token_stream(
        &proc_macro2::TokenStream::new(),
        parameters_token_stream,
        err_type_token_stream,
        ts,
    ));
    with_attr_token_stream(attr_token_stream, &ts0)
}
pub fn generate_pub_const_try_new_token_stream(
    attr_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    let ts0 = pub_space_token_stream(&generate_const_try_new_token_stream(
        &proc_macro2::TokenStream::new(),
        parameters_token_stream,
        err_type_token_stream,
        ts,
    ));
    with_attr_token_stream(attr_token_stream, &ts0)
}
pub fn generate_impl_try_new_for_identifier_token_stream(
    attr_token_stream: &dyn quote::ToTokens,
    identifier_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    impl_identifier_token_stream(
        identifier_token_stream,
        &generate_try_new_token_stream(
            attr_token_stream,
            parameters_token_stream,
            err_type_token_stream,
            ts,
        ),
    )
}
pub fn generate_impl_const_try_new_for_identifier_token_stream(
    attr_token_stream: &dyn quote::ToTokens,
    identifier_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    impl_identifier_token_stream(
        identifier_token_stream,
        &generate_const_try_new_token_stream(
            attr_token_stream,
            parameters_token_stream,
            err_type_token_stream,
            ts,
        ),
    )
}
pub fn generate_impl_pub_try_new_for_identifier_token_stream(
    attr_token_stream: &dyn quote::ToTokens,
    identifier_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    impl_identifier_token_stream(
        identifier_token_stream,
        &generate_pub_try_new_token_stream(
            attr_token_stream,
            parameters_token_stream,
            err_type_token_stream,
            ts,
        ),
    )
}
pub fn generate_impl_pub_const_try_new_for_identifier_token_stream(
    attr_token_stream: &dyn quote::ToTokens,
    identifier_token_stream: &dyn quote::ToTokens,
    parameters_token_stream: &dyn quote::ToTokens,
    err_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    impl_identifier_token_stream(
        identifier_token_stream,
        &generate_pub_const_try_new_token_stream(
            attr_token_stream,
            parameters_token_stream,
            err_type_token_stream,
            ts,
        ),
    )
}
#[cfg(test)]
mod tests {
    fn cmpct(v: &str) -> String {
        v.split_whitespace().collect::<String>()
    }
    fn empty_token_stream() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
    #[test]
    fn generate_impl_new_for_identifier_token_stream_generates_non_const_new() {
        let identifier: proc_macro2::TokenStream = "Cfg".parse().expect("48495be4");
        let parameters: proc_macro2::TokenStream = "v:usize".parse().expect("db75b4fb");
        let body: proc_macro2::TokenStream = "Self{v}".parse().expect("7ad6dd07");
        let ts = super::generate_impl_new_for_identifier_token_stream(
            &identifier,
            &empty_token_stream(),
            &parameters,
            &body,
        );
        assert_eq!(
            cmpct(&ts.to_string()),
            cmpct("impl Cfg { fn new(v:usize) -> Self { Self{v} } }")
        );
    }
    #[test]
    fn generate_impl_const_new_for_identifier_token_stream_generates_const_new() {
        let identifier: proc_macro2::TokenStream = "Cfg".parse().expect("7795af9b");
        let parameters: proc_macro2::TokenStream = "v:usize".parse().expect("28ccdfc4");
        let body: proc_macro2::TokenStream = "Self{v}".parse().expect("46fb1c80");
        let ts = super::generate_impl_const_new_for_identifier_token_stream(
            &identifier,
            &empty_token_stream(),
            &parameters,
            &body,
        );
        assert_eq!(
            cmpct(&ts.to_string()),
            cmpct("impl Cfg { const fn new(v:usize) -> Self { Self{v} } }")
        );
    }
    #[test]
    fn generate_impl_pub_const_new_for_identifier_token_stream_generates_pub_const_new() {
        let identifier: proc_macro2::TokenStream = "Cfg".parse().expect("4afbe04b");
        let attr: proc_macro2::TokenStream = "#[inline]".parse().expect("5cfde4dd");
        let parameters: proc_macro2::TokenStream = "v:usize".parse().expect("4304ab24");
        let body: proc_macro2::TokenStream = "Self{v}".parse().expect("29ac89d5");
        let ts = super::generate_impl_pub_const_new_for_identifier_token_stream(
            &identifier,
            &attr,
            &parameters,
            &body,
        );
        assert_eq!(
            cmpct(&ts.to_string()),
            cmpct("impl Cfg { #[inline] pub const fn new(v:usize) -> Self { Self{v} } }")
        );
    }
}
