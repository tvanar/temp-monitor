use core::fmt;
use dotenvy::dotenv;
use serde::Serialize;
use std::time::Duration;
use std::env;
use sysinfo::{Components, System};
use tokio::time::sleep;

#[derive(Serialize)]
pub struct Stats {
    hostname: String,
    cpu_usage: f32,
    cpu_freq: u64,
    ram_usage: u64,
    ram_total: u64,
    temp: f32,
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "host={} cpu_usage={} cpu_freq={} ram_usage={} ram_total={} temp={}",
            self.hostname, self.cpu_usage, self.cpu_freq, self.ram_usage, self.ram_total, self.temp
        )
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set in .env");

    // let client = reqwest::Client::new();

    let mut sys = System::new_all();
    let mut components = Components::new_with_refreshed_list();

    let hostname = System::host_name().unwrap_or_else(|| "unknown".into());

    sys.refresh_memory();
    let total_ram = sys.total_memory();

    let cpu_index = components
        .iter()
        .position(|c| c.label().to_lowercase().contains("cpu"));

    loop {
        sys.refresh_cpu_all();
        sys.refresh_memory();
        components.refresh(true);

        let cpu_usage = sys.global_cpu_usage();

        let cpu_freq = sys.cpus().first().map(|cpu| cpu.frequency()).unwrap_or(0);

        let ram_usage = sys.used_memory();

        let temp = cpu_index
            .and_then(|i| components.get(i))
            .and_then(|c| c.temperature())
            .or_else(|| {
                std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
                    .ok()
                    .and_then(|t| t.trim().parse::<f32>().ok())
                    .map(|t| t / 1000.0)
            })
            .unwrap_or(0.0);

        let stats = Stats {
            hostname: hostname.clone(),
            cpu_usage,
            cpu_freq,
            ram_usage,
            ram_total: total_ram,
            temp,
        };

        // if let Err(e) = client.post(&server_url).json(&stats).send().await {
        //     eprintln!("Failed to send stats: {}", e);
        // }

        println!("{}", stats);

        sleep(Duration::from_secs(5)).await;
    }
}
