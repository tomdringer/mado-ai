// mado-ai — AI chat terminal plugin for Mado
//
// Spawns an interactive login shell so you can run `claude`, `copilot`,
// `gemini`, or any other AI CLI. Mado hosts the PTY; this binary just
// hands control to the user's shell.

use std::os::unix::process::CommandExt;

fn main() {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string());

    // exec replaces this process — on success it never returns
    let err = std::process::Command::new(&shell).arg("-il").exec();
    eprintln!("mado-ai: failed to exec {shell}: {err}");
    std::process::exit(1);
}
