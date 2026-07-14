pub(super) fn run_commands(commands: &[(&str, &[&str])]) -> Result<(), ()> {
    commands.iter().try_fold(
        (),
        |(), (program, args)| match std::process::Command::new(program).args(*args).status() {
            Ok(status) if status.success() => Ok(()),
            Ok(status) => {
                super::reporting::command_failed(program, args, status);
                Err(())
            }
            Err(error) => {
                super::reporting::command_spawn_failed(program, args, &error);
                Err(())
            }
        },
    )
}
