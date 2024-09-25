use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type Stats = Arc<Mutex<HashMap<String, (usize, usize, usize)>>>;

fn ping(ip: String, stats: Stats) {
    loop {
        // Ejecutar el comando ping con los parámetros: 3 pings (-c 3) y 0.2 segundos entre ellos (-i 0.2)
        let output = Command::new("ping")
            .arg("-c")
            .arg("3")
            .arg("-i")
            .arg("0.2")
            .arg(&ip)
            .output();

        let success = output.map_or(false, |o| o.status.success());

        // Actualizar estadísticas
        let mut stats_guard = stats.lock().unwrap();
        let entry = stats_guard.entry(ip.clone()).or_insert((0, 0, 0));
        entry.0 += 1;  // Aumenta el número de intentos
        if success {
            entry.1 += 1;  // Aumenta el número de éxitos si el comando fue exitoso
        } else {
            entry.2 += 1;  // Aumenta el número de fallos en caso de error
        }

        // Esperar 1 segundo antes del siguiente ciclo de pings
        thread::sleep(Duration::from_secs(1));
    }
}

fn show_stats(stats: Stats) {
    loop {
        thread::sleep(Duration::from_secs(10)); // Mostrar estadísticas cada 10 segundos
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
    // Obtener las IPs de los argumentos de entrada
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Please provide at least one IP address.");
        return;
    }

    // Crear un Arc y Mutex para las estadísticas compartidas
    let stats: Stats = Arc::new(Mutex::new(HashMap::new()));

    // Crear un hilo para cada IP
    for ip in &args {
        let stats_clone = Arc::clone(&stats);
        let ip_clone = ip.clone();
        thread::spawn(move || ping(ip_clone, stats_clone));
    }

    // Crear el hilo para mostrar las estadísticas
    let stats_clone = Arc::clone(&stats);
    thread::spawn(move || show_stats(stats_clone));

    // Mantener el programa corriendo
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
