use clap::{self, Parser};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
pub struct Options {
    #[clap(
        long = "ports",
        default_value = "8080,8081,8082",
        help = "ports to scan, separated by commas"
    )]
    pub ports: String,
    #[clap(
        long = "fast-response-time",
        default_value = "0.04",
        help = "considering fast response time [s]"
    )]
    pub fast_response_time: f32,
    #[clap(long = "output-folder", default_value = "LM-servers")]
    pub output_dir: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostBody {
    pub temperature: f32,
    pub max_tokens: i32,
    pub stream: bool,
    pub messages: Vec<Message>,
}
