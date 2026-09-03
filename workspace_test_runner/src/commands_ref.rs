#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct CommandsRef<'commands_lt>(
    &'commands_lt [(&'commands_lt str, &'commands_lt [&'commands_lt str])],
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
