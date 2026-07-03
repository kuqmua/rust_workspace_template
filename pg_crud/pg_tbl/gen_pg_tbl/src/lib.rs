#[proc_macro_attribute]
pub fn cm_er_vrts(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn cm_logic(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn cmn_er_vrts(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn cmn_logic(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn co_er_vrts(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn co_logic(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn dlo_er_vrts(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn dlo_logic(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn dm_er_vrts(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn dm_logic(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn gen_pg_tbl_config(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_derive(GenPgTbl, attributes(gen_pg_tbl_pk))]
pub fn gen_pg_tbl(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_pg_tbl_src::gen_pg_tbl(input_token_stream.into()).into()
}

#[proc_macro_attribute]
pub fn rm_er_vrts(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn rm_logic(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn ro_er_vrts(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn ro_logic(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn um_er_vrts(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn um_logic(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn uo_er_vrts(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_attribute]
pub fn uo_logic(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}
