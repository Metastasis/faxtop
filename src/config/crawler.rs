use crate::config::config;
use crate::config::config::{get_selector, get_selector2};

pub struct Crawler {}

pub struct SiteObjectModel {
    pub data: serde_json::Map<String, serde_json::Value>,
}

impl SiteObjectModel {
    fn new() -> SiteObjectModel {
        SiteObjectModel {
            data: serde_json::Map::new(),
        }
    }
}

impl Crawler {
    pub async fn run(config: &config::CrawlConfig) -> SiteObjectModel {
        let mut site_model = SiteObjectModel::new();
        for (page_name, page) in &config.pages {
            let url = config.meta.url.as_str().to_owned() + page.path.as_str();
            let html = reqwest::get(&url).await.unwrap().text().await.unwrap();
            let document = scraper::Html::parse_document(html.as_str());
            let mut d = serde_json::Value::Object(serde_json::Map::new());
            parse_page(&mut d, &page, &document);
            site_model.data.insert(page_name.to_owned(), d);
        }
        site_model
    }
}

fn parse_page(mut result: &mut serde_json::Value, page: &config::Page, document: &scraper::Html) {
    for (value_name, value) in &page.data {
        parse_section(&mut result, value_name, value, &document.root_element());
    }
}

fn parse_section(
    result: &mut serde_json::Value,
    section_name: &str,
    section: &config::Value,
    document: &scraper::ElementRef,
) {
    match section {
        config::Value::Nested(obj) => {
            let css_selector = get_selector(section);
            let selector = scraper::Selector::parse(css_selector).unwrap();
            let mut fragments = document.select(&selector);
            let size = fragments.clone().count();
            if size > 1 {
                let mut acc: Vec<serde_json::Value> = Vec::new();
                for element_ref in fragments {
                    let mut acc_element = serde_json::Value::Object(serde_json::Map::new());
                    for (value_name, value) in &obj.fields {
                        parse_section(&mut acc_element, value_name, &value, &element_ref);
                    }
                    acc.push(acc_element);
                }
                let val = serde_json::Value::Array(acc);
                match result {
                    serde_json::Value::Object(o) => {
                        o.insert(section_name.to_owned(), val);
                    }
                    serde_json::Value::Array(a) => a.push(val),
                    _ => unimplemented!(),
                }
            } else if size == 1 {
                let acc = serde_json::Map::new();
                let mut val = serde_json::Value::Object(acc);
                let element_ref = fragments.next().unwrap();
                for (value_name, value) in &obj.fields {
                    parse_section(&mut val, value_name, &value, &element_ref);
                }
                match result {
                    serde_json::Value::Object(o) => {
                        o.insert(section_name.to_owned(), val);
                    }
                    serde_json::Value::Array(a) => a.push(val),
                    _ => unimplemented!(),
                }
            }
        }
        config::Value::Leaf(obj) => {
            let css_selector = get_selector2(&obj.selector);
            let selector = scraper::Selector::parse(css_selector).unwrap();
            let mut fragments = document.select(&selector);
            if let Some(v) = fragments.next() {
                let r = serde_json::Value::String(v.inner_html());
                match result {
                    serde_json::Value::Object(o) => {
                        o.insert(section_name.to_owned(), r);
                    }
                    serde_json::Value::Array(a) => a.push(r),
                    _ => unimplemented!(),
                }
            }
        }
    };
}
