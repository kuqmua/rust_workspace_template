#[derive(Debug, Clone, Copy)]
pub struct Disabled;

#[must_use]
pub const fn panic_if_err() -> Disabled {
    Disabled
}
