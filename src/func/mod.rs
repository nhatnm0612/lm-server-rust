use std::{
    fs::{self, DirEntry},
    io::{Result, Write},
    path::{Path, PathBuf},
    time,
};

use colored::Colorize;
use local_ip_address::local_ip;
use reqwest::{self, header::CONTENT_TYPE};

use lm_server::{Message, PostBody};

pub fn ensure_output_dir(path: &str) -> Result<()> {
    if !Path::new(path).is_dir() {
        fs::create_dir_all(path)?;
    }
    return Ok(());
}

pub fn remove_old_files(path: &str) -> Result<()> {
    if Path::new(path).is_file() {
        return Ok(());
    }
    let my_local_ip_addr: String = local_ip_address_formated();
    for child in fs::read_dir(path)? {
        let child: DirEntry = child.unwrap();
        let path: PathBuf = child.path();
        if path.is_dir() {
            continue;
        }
        let file_name: String = path.file_name().unwrap().to_str().unwrap().to_string();
        if file_name.starts_with(&my_local_ip_addr) {
            fs::remove_file(path).unwrap();
        }
    }
    Ok(())
}

pub fn local_ip_address_formated() -> String {
    let local_ip_addr: std::net::IpAddr = local_ip().unwrap();
    let local_ip_addr_str: String = local_ip_addr.to_string();
    return local_ip_addr_str.replace(".", "_");
}

pub async fn scanning_port(port: i32, output_dir: &str, fast_response_time: f32) -> () {
    let local_ip_addr_formated: String = local_ip_address_formated();
    let local_ip_addr: String = local_ip_addr_formated.replace("_", ".");
    let url: String = format!("http://{}:{}/v1/chat/completions/", local_ip_addr, port);
    let message: Message = Message {
        role: "user".to_string(),
        content: "what is the last letter of the alphabet?".to_string(),
    };
    let post_body: PostBody = PostBody {
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
            let elapse_time: f32 = now.elapsed().as_micros() as f32;
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
            let file_name: String =
                format!("{}_{}_{}.txt", local_ip_addr_formated, port, elapse_time);
            let path: String = format!("{}/{}", output_dir, file_name);
            let mut file: fs::File = fs::File::create(path).unwrap();
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
        fs::create_dir_all("./rstests").unwrap();
        fs::File::create(format!("./rstests/{}_1234_1.txt", ip_address_formated)).unwrap();
        fs::File::create(format!("./rstests/{}_1235_2.txt", ip_address_formated)).unwrap();
        remove_old_files("./rstests").unwrap();
        assert!(fs::read_dir("./rstests").unwrap().collect::<Vec<_>>().len() == 0);
        fs::remove_dir_all("./rstests").unwrap();
    }
}
