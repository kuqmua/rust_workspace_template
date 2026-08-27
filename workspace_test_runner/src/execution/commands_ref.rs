// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::field_scoped_visibility_modifiers)] // split wrapper representation is consumed only by its parent execution facade
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct CommandsRef<'commands_lt>(
    pub(super) &'commands_lt [(&'commands_lt str, &'commands_lt [&'commands_lt str])],
);
impl<'commands_lt, const N: usize>
    From<&'commands_lt [(&'commands_lt str, &'commands_lt [&'commands_lt str]); N]>
    for CommandsRef<'commands_lt>
{
    fn from(
        value: &'commands_lt [(&'commands_lt str, &'commands_lt [&'commands_lt str]); N],
    ) -> Self {
        Self(value.as_slice())
    }
}
