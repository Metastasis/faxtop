use crate::config::{config::CrawlConfig, crawler::Crawler};
use std::convert::TryFrom;
use std::path;

#[tokio::main]
async fn parse_config() {
    let path_to_cfg = path::Path::new(file!())
        .parent()
        .unwrap()
        .join("../artifacts/config.json");
    // println!("{}", path_to_cfg.to_str().unwrap());
    let crawl_cfg = CrawlConfig::try_from(path_to_cfg).unwrap();
    let site_model = Crawler::run(&crawl_cfg).await;
    println!("{:#?}", site_model.data);
    // db.save(parsed_site.into()).await;
}
