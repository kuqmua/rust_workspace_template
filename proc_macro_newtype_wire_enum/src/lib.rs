#[proc_macro_derive(WireEnum, attributes(wire_enum, wire))]
pub fn wire_enum(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::wire_enum(token_stream.into()).into()
}
