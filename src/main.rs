use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type Stats = Arc<Mutex<HashMap<String, (usize, usize, usize)>>>;

fn ping(ip: String, stats: Stats) {
    loop {
        let output = Command::new("ping")
            .arg("-c")
            .arg("3")
            .arg("-i")
            .arg("0.2")
            .arg(&ip)
            .output();

        let success = output.map_or(false, |o| o.status.success());

        let mut stats_guard = stats.lock().unwrap();
        let entry = stats_guard.entry(ip.clone()).or_insert((0, 0, 0));
        entry.0 += 1;
        if success {
            entry.1 += 1;
        } else {
            entry.2 += 1;
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn show_stats(stats: Stats) {
    loop {
        thread::sleep(Duration::from_secs(10));
        let stats_guard = stats.lock().unwrap();

        println!("IP Statistics:");
        for (ip, (attempts, success, failures)) in stats_guard.iter() {
            println!(
                "IP: {} | Attempts: {} | Success: {} | Failures: {}",
                ip, attempts, success, failures
            );
        }
        println!("------------------------------------");
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Please provide at least one IP address.");
        return;
    }

    let stats: Stats = Arc::new(Mutex::new(HashMap::new()));

    for ip in &args {
        let stats_clone = Arc::clone(&stats);
        let ip_clone = ip.clone();
        thread::spawn(move || ping(ip_clone, stats_clone));
    }

    let stats_clone = Arc::clone(&stats);
    thread::spawn(move || show_stats(stats_clone));

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
