use std::process::Command;
use colored::Colorize;

pub fn run_command(message: &str, program: &str, args: &[&str]) {
    println!("{}", message.cyan());

    let status = Command::new(program)
        .args(args)
        .status();

    match status {
        Ok(s) if s.success() => println!("{}", format!("{} finished successfully.", program).green()),
        Ok(s) => eprintln!("{}", format!("{} exited with status: {}", program, s).yellow()),
        Err(e) => eprintln!("{}", format!("Failed to run {}: {}", program, e).red()),
    }
}
