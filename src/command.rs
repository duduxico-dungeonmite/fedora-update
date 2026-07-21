use std::process::Command;

pub fn run_command(message: &str, program: &str, args: &[&str]) {
    println!("{}", message);

    let status = Command::new(program)
        .args(args)
        .status();

    match status {
        Ok(s) if s.success() => println!("{} finished successfully.", program),
        Ok(s) => eprintln!("{} exited with status: {}", program, s),
        Err(e) => eprintln!("Failed to run {}: {}", program, e),
    }
}