mod keyword {
    syn::custom_keyword!(constants);
    syn::custom_keyword!(fragments);
}

#[path = "collection_max_len.rs"]
mod collection_max_len;
#[path = "constant.rs"]
mod constant;
#[path = "constant_part.rs"]
mod constant_part;
#[path = "constant_parts.rs"]
mod constant_parts;
#[path = "constants.rs"]
mod constants;
#[path = "define_str_constants_input.rs"]
mod define_str_constants_input;
#[path = "fragment.rs"]
mod fragment;
#[path = "fragments.rs"]
mod fragments;
#[path = "syn_ident.rs"]
mod syn_ident;
#[path = "syn_lit_str.rs"]
mod syn_lit_str;
#[path = "syn_visibility.rs"]
mod syn_visibility;

use collection_max_len::COLLECTION_MAX_LEN;
use constant::Constant;
use constant_part::ConstantPart;
use constant_parts::ConstantParts;
use constants::Constants;
pub(crate) use define_str_constants_input::DefineStrConstantsInput;
use fragment::Fragment;
use fragments::Fragments;
use syn_ident::SynIdent;
use syn_lit_str::SynLitStr;
use syn_visibility::SynVisibility;
