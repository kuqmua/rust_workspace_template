#[path = "location/compile_error_message.rs"]
mod compile_error_message;
#[path = "location/compile_error_token_stream.rs"]
mod compile_error_token_stream;
#[path = "location/generate_serde_version_of_named_syn_variant.rs"]
mod generate_serde_version_of_named_syn_variant;
#[path = "location/location_field_attr.rs"]
mod location_field_attr;
#[path = "location/syn_variant_ref.rs"]
mod syn_variant_ref;

pub use generate_serde_version_of_named_syn_variant::generate_serde_version_of_named_syn_variant;
pub use location_field_attr::LocationFieldAttr;
pub use syn_variant_ref::SynVariantRef;
