use std::process::Command;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
pub fn reboot_check() {
    let status = Command::new("needs-restarting")
        .args(["-r"])
        .status();

    match status {
        Ok(s) => match s.code() {
            Some(0) => {
                println!("Reboot check completed, no need to reboot!");
            }
            Some(1) => {
                print!("Reboot check completed, needs to reboot. Proceed? [y/N]: ");
                io::stdout().flush().unwrap();
                let mut answer = String::new();
                io::stdin().read_line(&mut answer).unwrap();

                let answer = answer.trim().to_lowercase();

                if answer == "y" || answer == "yes" {
                    for i in (1..=5).rev() {
                    println!("Rebooting in {} seconds. Press Ctrl + C to quit.", i);
                        thread::sleep(Duration::from_secs(1));
                    }
                    let _ = Command::new("sudo").arg("reboot").status();
                } else {
                    println!("Some core packages may not work properly while in this session. Rebooting is highly recommended.");
                }
            }
            Some(code) => {
                eprintln!("needs-restarting exited with unexpected code: {}", code);
            }
            None => {
                eprintln!("Process terminated without an exit code.");
            }
        },
        Err(e) => {
            eprintln!("Failed to run needs-restarting: {}", e);
        }
    }
}