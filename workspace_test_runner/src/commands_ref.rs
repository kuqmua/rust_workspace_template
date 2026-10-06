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

#[cfg(test)]
mod tests {
    #[test]
    fn test_command_array_conversion_preserves_order_and_nested_borrowing() {
        let arguments = [constants_str::CHECK, constants_str::STATIC];
        let commands = [
            (constants_str::STATIC, arguments.as_slice()),
            (constants_str::DATABASE, &[]),
            (constants_str::STATIC, arguments.as_slice()),
        ];
        let commands_ref = crate::commands_ref::CommandsRef::from(&commands);
        assert_eq!(*commands_ref, commands.as_slice());
        assert!(std::ptr::eq(*commands_ref, commands.as_slice()));
        assert!(
            commands_ref
                .first()
                .is_some_and(|command| { std::ptr::eq(command.1, arguments.as_slice()) })
        );
        assert!(crate::commands_ref::CommandsRef::from(&[]).is_empty());
    }
}
