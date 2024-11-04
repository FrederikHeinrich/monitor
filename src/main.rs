mod config;

use config::AppConfig;
use config::load_config;
use sysinfo::{System, Networks};
use influxdb::{Client, InfluxDbWriteable, WriteQuery};
use std::time::Duration;
use chrono::Utc;
use tokio::time::sleep;
use tracing::{info, error, instrument};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Konfiguration laden
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {:?}", e);
            return;
        }
    };

    info!("Starting system agent...");
    collect_and_push_data(config).await;
}

#[instrument]
async fn collect_and_push_data(config: AppConfig) {
    let client = Client::new(&config.influx_url, &config.influx_bucket)
        .with_token(&config.influx_token);

    let mut system = System::new_all();
    let mut networks = Networks::new_with_refreshed_list();

    loop {
        system.refresh_all();
        networks.refresh();

        // Systemdaten sammeln
        let cpu_usage = system.global_cpu_usage();
        let total_memory = system.total_memory() / 1000000;
        let used_memory = system.used_memory() / 1000000;
        let total_swap = system.total_swap() / 1000000;
        let used_swap = system.used_swap() / 1000000;

        // Daten als InfluxDB Query erstellen
        let mut write_query = WriteQuery::new(Utc::now().into(), "system_monitor")
            .add_tag("identifier", &*config.identifier)
            .add_field("cpu_usage", cpu_usage)
            .add_field("total_memory", total_memory)
            .add_field("used_memory", used_memory)
            .add_field("total_swap", total_swap)
            .add_field("used_swap", used_swap);

        // Netzwerkdaten hinzufügen
        for (interface_name, data) in &networks {
            write_query = write_query
                .add_field(&format!("{}_received", interface_name), data.received() as i64)
                .add_field(&format!("{}_transmitted", interface_name), data.transmitted() as i64)
                .add_field(&format!("{}_received_total", interface_name), data.total_received() as i64)
                .add_field(&format!("{}_transmitted_total", interface_name), data.total_transmitted() as i64);
        }

        // Daten in InfluxDB schreiben
        match client.query(&write_query).await {
            Ok(ok) => info!("Successfully pushed data to InfluxDB. ( {} )", ok),
            Err(e) => error!("Failed to push data to InfluxDB: {:?}", e),
        }

        // Wartezeit zwischen den Updates
        sleep(Duration::from_secs(config.refresh_rate)).await;
    }
}
