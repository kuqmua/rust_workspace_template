#[proc_macro_derive(AsRefStrEnumWithUnitFieldsToSnakeCaseStr)]
pub fn as_ref_str_enum_with_unit_fields_to_snake_case_str(
    token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_naming_shared::as_ref_str_enum_with_unit_fields_to_snake_case_str(
        token_stream.into(),
    )
    .into()
}
