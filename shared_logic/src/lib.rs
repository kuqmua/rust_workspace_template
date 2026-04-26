#![forbid(unsafe_code)]

#[must_use]
pub const fn hello() -> &'static str {
    "hello"
}
