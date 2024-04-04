use std::{fs, io::Write, path::Path, thread, time};

use clap::{self, Parser};
use colored::Colorize;
use local_ip_address::local_ip;
use reqwest::{self, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Parser)]
struct Options {
    #[clap(
        long = "ports",
        default_value = "1234,1235,1236",
        help = "ports to scan, separated by commas"
    )]
    ports: String,
    #[clap(
        long = "fast-response-time",
        default_value = "0.02",
        help = "considering fast response time [s]"
    )]
    fast_response_time: f32,
    #[clap(long = "output-folder", default_value = "T:/LM-servers")]
    output_dir: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PostBody {
    temperature: f32,
    max_token: i32,
    stream: bool,
    messages: Vec<Message>,
}

fn ensure_output_dir(path: &str) -> std::io::Result<()> {
    if !Path::new(path).is_dir() {
        fs::create_dir_all(path)?;
    }
    return Ok(());
}

fn local_ip_address_formated() -> String {
    let local_ip_addr: std::net::IpAddr = local_ip().unwrap();
    let local_ip_addr_str: String = local_ip_addr.to_string();
    return local_ip_addr_str.replace(".", "_");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse();
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

fn remove_old_files(path: &str) -> std::io::Result<()> {
    if Path::new(path).is_file() {
        return Ok(());
    }
    let my_local_ip_addr = local_ip_address_formated();
    for child in fs::read_dir(path)? {
        let child = child.unwrap();
        let path = child.path();
        if path.is_dir() {
            continue;
        }
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        if file_name.starts_with(&my_local_ip_addr) {
            fs::remove_file(path).unwrap();
        }
    }
    Ok(())
}

async fn scanning_port(port: i32, output_dir: &str, fast_response_time: f32) -> () {
    let local_ip_addr_formated = local_ip_address_formated();
    let local_ip_addr = local_ip_addr_formated.replace("_", ".");
    let url = format!("http://{}:{}/v1/chat/completions/", local_ip_addr, port);
    let message = Message {
        role: "user".to_string(),
        content: "what is the last letter of the alphabet?".to_string(),
    };
    let post_body = PostBody {
        temperature: 0.1,
        max_token: 12,
        stream: false,
        messages: vec![message],
    };
    let now = time::Instant::now();
    match reqwest::Client::new()
        .post(format!("http://127.0.0.1:{}/v1/chat/completions/", port))
        .header(CONTENT_TYPE, "application/json")
        .json(&post_body)
        .send()
        .await
    {
        Ok(_) => {
            let elapse_time = now.elapsed().as_micros() as f32;
            let elapse_time: f32 = elapse_time / 1_000_000.0;
            if elapse_time <= fast_response_time {
                println!(
                    "{} {} {} {} {}",
                    "> Response from".green(),
                    &url.green().bold(),
                    "takes".green(),
                    elapse_time.to_string().green().bold(),
                    "second[s]".green()
                );
            } else {
                println!(
                    "{} {} {} {} {}",
                    "> Response from".yellow(),
                    &url.yellow().bold(),
                    "takes".yellow(),
                    elapse_time.to_string().yellow().bold(),
                    "second[s]".yellow()
                );
            }
            let file_name = format!("{}_{}_{}.txt", local_ip_addr_formated, port, elapse_time);
            let path = format!("{}/{}", output_dir, file_name);
            let mut file = fs::File::create(path).unwrap();
            file.write(format!("HOST: {}\n", local_ip_addr).as_bytes())
                .unwrap();
            file.write(format!("PORT: {}\n", port).as_bytes()).unwrap();
            file.write(format!("RESPONSE TIME: {}\n", elapse_time).as_bytes())
                .unwrap();
        }
        Err(_) => {
            println!("{} {}", "> No response from".red(), url.red().bold());
        }
    }
}

#[cfg(test)]
mod test {
    use std::{fs, path::Path};

    use super::*;

    #[test]
    fn test_ensure_output_dir() {
        let path = Path::new("./test");
        assert_eq!(path.is_dir(), false);
        match ensure_output_dir("./test") {
            Ok(_) => (),
            Err(e) => println!("Error {:?}", e),
        }
        assert_eq!(path.is_dir(), true);
        match ensure_output_dir("./test") {
            Ok(_) => (),
            Err(e) => println!("Error {:?}", e),
        }
        assert_eq!(path.is_dir(), true);
        fs::remove_dir_all("./test").unwrap();
    }

    #[test]
    fn test_remove_old_files() {
        let ip_address_formated: String = local_ip_address_formated();
        fs::create_dir_all("./tests").unwrap();
        fs::File::create(format!("./tests/{}_1234_1.txt", ip_address_formated)).unwrap();
        fs::File::create(format!("./tests/{}_1235_2.txt", ip_address_formated)).unwrap();
        remove_old_files("./tests").unwrap();
        assert!(fs::read_dir("./tests").unwrap().collect::<Vec<_>>().len() == 0);
        fs::remove_dir_all("./tests").unwrap();
    }
}
