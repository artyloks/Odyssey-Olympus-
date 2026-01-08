use std::process::{Command, Child};
use std::time::{Duration, Instant};
use std::thread;
use std::fs;
use std::path::Path;

// CONFIGURATION
const BRAIN_PATH: &str = "brain.py";
const KILL_SWITCH: &str = "/tmp/olympus_override"; 
const HEARTBEAT_MS: u64 = 500;

fn spawn_brain() -> Child {
    Command::new("python3")
        .arg(BRAIN_PATH)
        .spawn()
        .expect(">>> [CRITICAL] Failed to spawn Brain.")
}

fn main() {
    println!("--- OLYMPUS SOVEREIGN CLOUD: ONLINE ---");
    // PRE-FLIGHT CHECK
    if !Path::new(BRAIN_PATH).exists() {
        panic!(">>> [FATAL] Brain file '{}' missing. Aborting.", BRAIN_PATH);
    }

    let mut brain = spawn_brain();
    let mut last_check = Instant::now();

    loop {
        // 1. HARDWARE OVERRIDE CHECK
        if Path::new(KILL_SWITCH).exists() {
             println!(">>> [OVERRIDE] Kill signal detected.");
             let _ = brain.kill();
             std::process::exit(0);
        }

        // 2. PROCESS MONITORING
        match brain.try_wait() {
            Ok(Some(_)) => {
                println!(">>> [FAILURE] Brain died. Restarting...");
                thread::sleep(Duration::from_millis(100)); 
                brain = spawn_brain();
            }
            Ok(None) => {
                thread::sleep(Duration::from_millis(HEARTBEAT_MS));
            }
            Err(_) => {
                brain = spawn_brain();
            }
        }
    }
}
