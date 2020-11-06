mod config;

use config::{CrawlConfig, Value};
use reqwest;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path;

#[tokio::main]
async fn main() {
    let path_to_cfg = path::Path::new(file!())
        .parent()
        .unwrap()
        .join("../artifacts/config.json");
    println!("{}", path_to_cfg.to_str().unwrap());
    let result = CrawlConfig::try_from(path_to_cfg).unwrap();
    // println!("{:#?}", result);
    let ideas = result.pages.get("currencyIdeas").unwrap();
    let advises = ideas.data.get("advises").unwrap();
    let url = result.meta.url.as_str().to_owned() + ideas.path.as_str();
    let html = reqwest::get(&url).await.unwrap().text().await.unwrap();
    let document = Html::parse_document(html.as_str());
    let css_selector = get_selector(advises);
    let selector = Selector::parse(css_selector).unwrap();
    println!("{}", url);
    // println!("{}", css_selector);
    // println!("{:?}", document);
    let fragments = document.select(&selector);
    for fragment in fragments {
        match advises {
            Value::Nested(obj) => {
                let mut r: HashMap<&str, String> = HashMap::new();
                for (key, value) in &obj.fields {
                    match value {
                        Value::Nested(_obj) => unimplemented!(),
                        Value::Leaf(obj) => {
                            let slctr_str = get_selector2(&obj.selector);
                            let slctr = Selector::parse(slctr_str).unwrap();
                            let part = fragment.select(&slctr);
                            part.for_each(|el| {
                                r.insert(key, el.inner_html());
                            })
                        }
                    }
                }
                println!("{:#?}", r);
            }
            Value::Leaf(_obj) => unimplemented!(),
        }
    }
}

fn get_selector(page_part: &Value) -> &str {
    match page_part {
        Value::Nested(obj) => match &obj.selector {
            config::Selector::Css(s) => s.as_str(),
        },
        Value::Leaf(obj) => match &obj.selector {
            config::Selector::Css(s) => s.as_str(),
        },
    }
}

fn get_selector2(selector: &config::Selector) -> &str {
    match selector {
        config::Selector::Css(s) => s,
    }
}
