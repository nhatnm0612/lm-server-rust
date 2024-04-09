use std::{fs, io::Write, time};

use colored::Colorize;
use local_ip_address::local_ip;
use reqwest::{self, header::CONTENT_TYPE};

use lm_server::{Message, PostBody};

pub fn local_ip_address_formated() -> String {
    let local_ip_addr: std::net::IpAddr = local_ip().unwrap();
    let local_ip_addr_str: String = local_ip_addr.to_string();
    return local_ip_addr_str.replace(".", "_");
}

pub async fn scanning_port(port: i32, output_dir: &str, fast_response_time: f32) -> () {
    let local_ip_addr_formated = local_ip_address_formated();
    let local_ip_addr = local_ip_addr_formated.replace("_", ".");
    let url = format!("http://{}:{}/v1/chat/completions/", local_ip_addr, port);
    let message = Message {
        role: "user".to_string(),
        content: "what is the last letter of the alphabet?".to_string(),
    };
    let post_body = PostBody {
        temperature: 0.1,
        max_tokens: 12,
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
