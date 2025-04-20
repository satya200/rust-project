use std::process::Command;

fn main() {
    let output = Command::new("/mnt/rust-dev/script_call_rust/myscript.sh")
        .arg("something")
        .output()
        .expect("Failed to execute script");

    let exit_code = output.status.code().unwrap_or(-1); // Fallback -1 if no exit code (e.g. terminated by signal)

    println!("Script exited with status code: {}", exit_code);

    if exit_code == 1 {
        println!("Script success (custom logic: exit code 1 = success)");
    } else {
        println!("Script failed");
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}

