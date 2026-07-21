#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct StdPathRef<'lt>(&'lt std::path::Path);
#[derive(Debug, newtype::FromInner)]
struct StdProcessCommand(std::process::Command);
#[derive(Debug)]
struct StdOsString(std::ffi::OsString);
impl From<&str> for StdOsString {
    fn from(value: &str) -> Self {
        Self(std::ffi::OsString::from(value))
    }
}
#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct ToolProgramRef<'lt>(&'lt str);
#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct ToolArgRef<'lt>(&'lt str);
#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct ToolArgsRef<'lt>(&'lt [&'lt str]);
#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct ToolEnvKeyRef<'lt>(&'lt str);
#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct ToolEnvValueRef<'lt>(&'lt str);
#[derive(Clone, Copy, Debug, newtype::DerefInner, newtype::Display, newtype::FromInner)]
pub struct StdProcessExitStatus(std::process::ExitStatus);
#[derive(Debug, newtype::AsRefOwned, newtype::DerefInner, newtype::FromInner)]
pub struct StdProcessOutput(std::process::Output);
pub struct ToolCommand {
    inner: StdProcessCommand,
    program: StdOsString,
}
impl std::fmt::Debug for ToolCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(str_constants::TOOLCOMMAND)
            .field(str_constants::PROGRAM, &self.program.0)
            .field(str_constants::ARGUMENTS, &str_constants::REDACTED)
            .finish_non_exhaustive()
    }
}
impl ToolCommand {
    pub fn arg(&mut self, value: ToolArgRef<'_>) -> &mut Self {
        let _command = self.inner.0.arg(value.0);
        self
    }
    pub fn args(&mut self, values: ToolArgsRef<'_>) -> &mut Self {
        let _command = self.inner.0.args(values.0);
        self
    }
    pub fn current_dir(&mut self, value: StdPathRef<'_>) -> &mut Self {
        let _command = self.inner.0.current_dir(value.0);
        self
    }
    pub fn env(&mut self, key: ToolEnvKeyRef<'_>, value: ToolEnvValueRef<'_>) -> &mut Self {
        let _command = self.inner.0.env(key.0, value.0);
        self
    }
    #[must_use]
    pub fn new(program: ToolProgramRef<'_>) -> Self {
        Self {
            inner: StdProcessCommand::from(std::process::Command::new(program.0)),
            program: StdOsString::from(program.0),
        }
    }
    pub fn output(&mut self) -> std::io::Result<StdProcessOutput> {
        self.inner.0.output().map(StdProcessOutput)
    }
    pub fn status(&mut self) -> std::io::Result<StdProcessExitStatus> {
        self.inner.0.status().map(StdProcessExitStatus)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn debug_redacts_arguments() {
        let mut command =
            super::ToolCommand::new(super::ToolProgramRef::from(str_constants::PRINTF));
        let _command = command.arg(super::ToolArgRef::from(str_constants::SECRET_VALUE));
        let debug = format!("{command:?}");
        assert!(debug.contains("printf"));
        assert!(debug.contains("<redacted>"));
        assert!(!debug.contains("secret-value"));
    }
}
