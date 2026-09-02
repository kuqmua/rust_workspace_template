fn cmpct(str: &str) -> String {
    str.split_whitespace().collect::<String>()
}
fn empty_token_stream() -> proc_macro2::TokenStream {
    proc_macro2::TokenStream::new()
}
#[test]
fn test_generate_impl_new_for_identifier_token_stream_generates_non_const_new() {
    let identifier: proc_macro2::TokenStream = constants_str::CFG
        .parse()
        .expect(constants_str::DIAGNOSTIC_48495BE4);
    let parameters: proc_macro2::TokenStream = constants_str::V_USIZE
        .parse()
        .expect(constants_str::DIAGNOSTIC_DB75B4FB);
    let body: proc_macro2::TokenStream = constants_str::SELF_V
        .parse()
        .expect(constants_str::DIAGNOSTIC_7AD6DD07);
    let ts = crate::generate_impl_new_for_identifier_token_stream_impl::generate_impl_new_for_identifier_token_stream_impl(
        &identifier,
        &empty_token_stream(),
        &parameters,
        &body,
    );
    assert_eq!(cmpct(&ts.to_string()), cmpct(constants_str::VALUE_87685B6B));
}
#[test]
fn test_generate_impl_const_new_for_identifier_token_stream_generates_const_new() {
    let identifier: proc_macro2::TokenStream = constants_str::CFG
        .parse()
        .expect(constants_str::DIAGNOSTIC_7795AF9B);
    let parameters: proc_macro2::TokenStream = constants_str::V_USIZE
        .parse()
        .expect(constants_str::DIAGNOSTIC_28CCDFC4);
    let body: proc_macro2::TokenStream = constants_str::SELF_V
        .parse()
        .expect(constants_str::DIAGNOSTIC_46FB1C80);
    let ts = crate::generate_impl_const_new_for_identifier_token_stream_impl::generate_impl_const_new_for_identifier_token_stream_impl(
        &identifier,
        &empty_token_stream(),
        &parameters,
        &body,
    );
    assert_eq!(cmpct(&ts.to_string()), cmpct(constants_str::VALUE_C3851857));
}
#[test]
fn test_generate_impl_pub_const_new_for_identifier_token_stream_generates_pub_const_new() {
    let identifier: proc_macro2::TokenStream = constants_str::CFG
        .parse()
        .expect(constants_str::DIAGNOSTIC_4AFBE04B);
    let attr: proc_macro2::TokenStream = constants_str::INLINE
        .parse()
        .expect(constants_str::DIAGNOSTIC_5CFDE4DD);
    let parameters: proc_macro2::TokenStream = constants_str::V_USIZE
        .parse()
        .expect(constants_str::DIAGNOSTIC_4304AB24);
    let body: proc_macro2::TokenStream = constants_str::SELF_V
        .parse()
        .expect(constants_str::DIAGNOSTIC_29AC89D5);
    let ts = crate::generate_impl_pub_const_new_for_identifier_token_stream_impl::generate_impl_pub_const_new_for_identifier_token_stream_impl(
        &identifier,
        &attr,
        &parameters,
        &body,
    );
    assert_eq!(cmpct(&ts.to_string()), cmpct(constants_str::VALUE_BA9AA4C0));
}
