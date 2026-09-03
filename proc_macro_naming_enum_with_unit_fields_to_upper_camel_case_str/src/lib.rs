#[proc_macro_derive(EnumWithUnitFieldsToUpperCamelCaseStr)]
pub fn enum_with_unit_fields_to_upper_camel_case_str(
    token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_naming_shared::enum_with_unit_fields_to_upper_camel_case_str(token_stream.into())
        .into()
}
