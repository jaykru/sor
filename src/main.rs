use std::env;
use std::io::{self, BufRead};
use std::process::Command;
fn main() {
    let mut tests: Vec<String> = env::args().collect();
    tests.remove(0);
    for filename in io::stdin().lock().lines() {
        if let Ok(filename) = filename {
            for test in &tests {
                let cmd = Command::new("bash").arg("-c").arg(format!("{} {}", test, filename)).spawn();
                if let Ok(mut cmd) = cmd {
                    if let Ok(status) = cmd.wait() {
                        if status.success() {
                            println!("{}", filename);
                        }
                    }
                }
            }
        }
    }
}