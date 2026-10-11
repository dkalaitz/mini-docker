use std::env;
use std::io;
use std::os::unix::process::CommandExt;
use std::process::{Command, ExitStatus};

use nix::mount::{MsFlags, mount};
use nix::sched::{CloneFlags, unshare};
use nix::unistd::sethostname;

pub(crate) fn init_child(program: &str, arguments: &[String]) -> io::Result<()> {
    println!("Container PID: {}", std::process::id());

    // Mount a fresh procfs
    mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::MS_NOSUID | MsFlags::MS_NOEXEC | MsFlags::MS_NODEV,
        None::<&str>,
    )?;

    let error = Command::new(program).args(arguments).exec();

    Err(error)
}

pub(crate) fn isolated_child(program: &str, arguments: &[String]) -> io::Result<()> {
    // Isolate hostname and mount points
    unshare(CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWNS)?;

    sethostname("minidock")?;

    // Prevent mount changes from propagating to other mount namespaces
    mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_REC | MsFlags::MS_PRIVATE,
        None::<&str>,
    )?;

    // Create a PID namespace for future children
    unshare(CloneFlags::CLONE_NEWPID)?;

    // Launch our internal init process
    let executable = env::current_exe()?;

    let mut child = Command::new(executable)
        .arg("_init")
        .arg(program)
        .args(arguments)
        .spawn()?;

    println!("Child PID (outside namespace): {}", child.id());

    let status = child.wait()?;

    std::process::exit(status.code().unwrap_or(1));
}

pub(crate) fn run_isolated(program: &str, arguments: &[String]) -> io::Result<ExitStatus> {
    let executable = env::current_exe()?;

    let mut child = Command::new(executable)
        .arg("_child")
        .arg(program)
        .args(arguments)
        .spawn()?;

    println!("Isolated child PID: {}", child.id());

    child.wait()
}
