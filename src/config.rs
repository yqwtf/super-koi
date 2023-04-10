
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    api_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {