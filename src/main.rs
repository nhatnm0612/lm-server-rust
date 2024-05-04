use std::{fs, path, process, thread, time};

use clap::Parser;
use colored::Colorize;
use file_mode::ModePath;
use tokio;

pub mod func;
use func::{ensure_output_dir, remove_old_files, scanning_port};

pub fn ensure_virtual_environment() -> () {
    let sh_file: String = "./ensure_venv.sh".to_string();
    let mut contents: String = "#! /bin/bash\n".to_string();
    contents += "python3 -m venv .venv\n";
    contents += "source .venv/bin/activate\n";
    contents += "python3 -m pip install --upgrade pip\n";
    contents += "python3 -m pip install -r dev/requirements.txt\n";
    fs::write(&sh_file, contents).unwrap();
    path::Path::new(&sh_file).set_mode("+x").unwrap();
    process::Command::new(&sh_file).spawn().unwrap();
}

pub fn run_fake_server(port: &String) -> () {
    println!(
        "{} {} {}",
        "\tDEBUG: <|start command|>".truecolor(105, 105, 105),
        "uvicorn dev.main:app -p".truecolor(105, 105, 105),
        port.truecolor(105, 105, 105)
    );
    let sh_file: String = format!("./fake_server_{}.sh", port);
    let mut contents: String = String::new();
    contents += "#! /bin/bash\n";
    contents += "source .venv/bin/activate\n";
    contents += format!("uvicorn dev.main:app --port {}\n", port).as_str();
    fs::write(&sh_file, contents).unwrap();
    path::Path::new(&sh_file).set_mode("+x").unwrap();
    process::Command::new(sh_file).spawn().unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options: lm_server::Options = lm_server::Options::parse();
    println!(
        "{} {}",
        "[*] Create output folder if possible:".blue().bold(),
        options.output_dir.blue()
    );
    let ports: Vec<String> = options.ports.split(",").map(|p| p.to_string()).collect();
    let environ: String = std::env::var("ENVIRON").unwrap_or("prod".to_string());
    if environ == "dev".to_string() {
        println!(
            "{}",
            "[*] Run fake Large Language Model Servers".blue().bold()
        );
        ensure_virtual_environment();
        thread::sleep(time::Duration::from_secs(30));
        for port in &ports {
            run_fake_server(port);
        }
        thread::sleep(time::Duration::from_secs(15));
    }
    ensure_output_dir(&options.output_dir)?;
    println!(
        "{} {}",
        "[*] Set fast response time:".blue().bold(),
        options.fast_response_time.to_string().blue()
    );
    loop {
        remove_old_files(&options.output_dir)?;
        for port in &ports {
            println!("{} {}", "[*] Scan port:".blue().bold(), port.blue());
            let port: i32 = port.parse::<i32>().unwrap();
            scanning_port(port, &options.output_dir, options.fast_response_time).await;
        }
        println!(
            "{}",
            "[*] Scanning completed, sleep for 120 seconds"
                .blue()
                .bold()
        );
        thread::sleep(time::Duration::from_secs(120));
    }
}
