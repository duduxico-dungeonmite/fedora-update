use std::process::Command;

//noinspection ALL,RsUnresolvedPath
pub fn update_dnf() {
    let update_msg = "Updating DNF...";

    // REMEMBER: when you release, remove "sudo" and only make:
    // Command::new("dnf5) | .args(["upgrade", "-y"]) | .status();
    // if someone reads this doc, every | = empty space/new line.
    // please remember! ><

    println!("{}", update_msg);

    let status = Command::new("sudo")
        .args(["dnf5", "upgrade", "-y"])
        .status();
    match status {
        Ok(s) if s.success() => println!("DNF update finished successfully."),
        Ok(s) => eprintln!("dnf5 exited with status: {}", s),
        Err(e) => eprintln!("Failed to run dnf5: {}", e),
    }
}

