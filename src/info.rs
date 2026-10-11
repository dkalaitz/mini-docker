use std::env;

pub(crate) fn show_info() {
    println!("MiniDocker Information");
    println!("OS: {}", env::consts::OS);
    println!("Architecture: {}", env::consts::ARCH);
    println!("PID: {}", std::process::id());
}
