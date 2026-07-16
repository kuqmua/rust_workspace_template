#[derive(
    Debug,
    Clone,
    Default,
    newtype::AsRefOwned,
    newtype::DerefInner,
    newtype::Display,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    newtype::ToTokens,
)]
pub struct GeneratedRustTokenStream(proc_macro2::TokenStream);
