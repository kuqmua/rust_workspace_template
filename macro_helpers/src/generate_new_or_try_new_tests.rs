fn cmpct(v: &str) -> String {
    v.split_whitespace().collect::<String>()
}
fn empty_token_stream() -> proc_macro2::TokenStream {
    proc_macro2::TokenStream::new()
}
#[test]
fn generate_impl_new_for_identifier_token_stream_generates_non_const_new() {
    let identifier: proc_macro2::TokenStream = constants_str::CFG.parse().expect("48495be4 generate_impl_new_for_identifier_token_stream_generates_non_const_new invariant must hold");
    let parameters: proc_macro2::TokenStream =
            constants_str::V_USIZE.parse().expect("db75b4fb generate_impl_new_for_identifier_token_stream_generates_non_const_new invariant must hold");
    let body: proc_macro2::TokenStream = constants_str::SELF_V.parse().expect("7ad6dd07 generate_impl_new_for_identifier_token_stream_generates_non_const_new invariant must hold");
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
    let identifier: proc_macro2::TokenStream = constants_str::CFG.parse().expect("7795af9b generate_impl_const_new_for_identifier_token_stream_generates_const_new invariant must hold");
    let parameters: proc_macro2::TokenStream =
            constants_str::V_USIZE.parse().expect("28ccdfc4 generate_impl_const_new_for_identifier_token_stream_generates_const_new invariant must hold");
    let body: proc_macro2::TokenStream = constants_str::SELF_V.parse().expect("46fb1c80 generate_impl_const_new_for_identifier_token_stream_generates_const_new invariant must hold");
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
    let identifier: proc_macro2::TokenStream = constants_str::CFG.parse().expect("4afbe04b generate_impl_pub_const_new_for_identifier_token_stream_generates_pub_const_new invariant must hold");
    let attr: proc_macro2::TokenStream = constants_str::INLINE.parse().expect("5cfde4dd generate_impl_pub_const_new_for_identifier_token_stream_generates_pub_const_new invariant must hold");
    let parameters: proc_macro2::TokenStream =
            constants_str::V_USIZE.parse().expect("4304ab24 generate_impl_pub_const_new_for_identifier_token_stream_generates_pub_const_new invariant must hold");
    let body: proc_macro2::TokenStream = constants_str::SELF_V.parse().expect("29ac89d5 generate_impl_pub_const_new_for_identifier_token_stream_generates_pub_const_new invariant must hold");
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
