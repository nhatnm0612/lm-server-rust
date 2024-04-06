use std::thread;

use clap::Parser;
use colored::Colorize;
use tokio;

pub mod func;
use func::not_testable::scanning_port;
use func::testable::{ensure_output_dir, remove_old_files};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = lm_server::Options::parse();
    println!(
        "{} {}",
        "[*] Create output folder if possible:".blue().bold(),
        options.output_dir.blue()
    );
    ensure_output_dir(&options.output_dir)?;
    println!(
        "{} {}",
        "[*] Set fast response time:".blue().bold(),
        options.fast_response_time.to_string().blue()
    );
    loop {
        remove_old_files(&options.output_dir)?;
        for port in options.ports.split(",") {
            println!("{} {}", "[*] Scan port:".blue().bold(), port.blue());
            let port = port.parse::<i32>().unwrap();
            scanning_port(port, &options.output_dir, options.fast_response_time).await;
        }
        println!(
            "{}",
            "[*] Scanning completed, sleep for 120 seconds"
                .blue()
                .bold()
        );
        thread::sleep(std::time::Duration::from_secs(120));
    }
}
