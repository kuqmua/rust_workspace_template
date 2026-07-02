#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SrcPlaceType {
    Github,
    Src,
}

impl SrcPlaceType {
    #[must_use]
    pub const fn default_value() -> Self {
        Self::Github
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TracingLevel {
    Debug,
    Error,
    Info,
    Trace,
    Warn,
}

impl TracingLevel {
    #[must_use]
    pub const fn default_value() -> Self {
        Self::Error
    }
}
