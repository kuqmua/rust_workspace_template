#[derive(Debug, Clone, Copy)]
pub struct AttrIdentText(&'static str);

impl AsRef<str> for AttrIdentText {
    fn as_ref(&self) -> &str {
        self.0
    }
}

pub trait AttrIdentStr {
    #[must_use]
    fn attr_ident_str(&self) -> AttrIdentText;
}
