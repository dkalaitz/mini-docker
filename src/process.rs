use std::io;
use std::process::{Command, ExitStatus};

pub(crate) fn run_command(program: &str, arguments: &[String]) -> io::Result<ExitStatus> {
    let mut child = Command::new(program).args(arguments).spawn()?;

    let status = child.wait()?;

    Ok(status)
}
