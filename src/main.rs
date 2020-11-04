mod config;

use config::CrawlConfig;
use serde_json;
use std::fs;
use std::path;

fn main() {
    let path_to_cfg = path::Path::new(file!()).parent().unwrap().join("../artifacts/config.json");
    println!("{}", path_to_cfg.to_str().unwrap());
    let config_str = fs::read_to_string(path_to_cfg).unwrap();
    let result: CrawlConfig = serde_json::from_str(config_str.as_str()).unwrap();
    println!("Hello, world!");
    println!("{:#?}", result);
}
