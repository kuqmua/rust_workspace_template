#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacroAttributeLookupError {
    AttributeNotList,
    NoAttribute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacroAttributePath;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacroAttribute;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacroAttributeMetaListTokenStream;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacroAttributeSearchOutcome {
    Found(MacroAttribute),
    NotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacroAttributeMetaListSearchOutcome {
    Found(MacroAttributeMetaListTokenStream),
    NotFound,
}
