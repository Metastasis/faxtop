use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert;
use std::fs;
use std::path;

#[derive(Serialize, Deserialize, Debug)]
pub enum Selector {
    #[serde(rename(deserialize = "@css"))]
    Css(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HtmlAttribute {
    #[serde(rename(deserialize = "@attr"))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "@type")]
pub enum Extractor {
    #[serde(rename(deserialize = "text"))]
    InnerText,
    #[serde(rename(deserialize = "attr"))]
    Attr(HtmlAttribute),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectLeaf {
    #[serde(rename(deserialize = "@selector"))]
    pub selector: Selector,
    #[serde(rename(deserialize = "@extract"))]
    pub extract: Extractor,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectComplex {
    #[serde(rename(deserialize = "@selector"))]
    pub selector: Selector,
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Value {
    #[serde(rename(deserialize = "nested"))]
    Nested(ObjectComplex),
    #[serde(rename(deserialize = "leaf"))]
    Leaf(ObjectLeaf),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    #[serde(rename(deserialize = "@path"))]
    pub path: String,
    #[serde(flatten)]
    pub data: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub version: u8,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CrawlConfig {
    #[serde(rename(deserialize = "@meta"))]
    pub meta: Meta,
    #[serde(flatten)]
    pub pages: HashMap<String, Page>,
}

impl convert::TryFrom<path::PathBuf> for CrawlConfig {
    type Error = serde_json::Error;

    fn try_from(path: path::PathBuf) -> Result<Self, Self::Error> {
        let config_str = fs::read_to_string(path).unwrap();
        serde_json::from_str(config_str.as_str())
    }
}

impl convert::TryFrom<&str> for CrawlConfig {
    type Error = serde_json::Error;

    fn try_from(config_str: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(config_str)
    }
}

pub fn get_selector(page_part: &Value) -> &str {
    match page_part {
        Value::Nested(obj) => match &obj.selector {
            Selector::Css(s) => s.as_str(),
        },
        Value::Leaf(obj) => match &obj.selector {
            Selector::Css(s) => s.as_str(),
        },
    }
}

pub fn get_selector2(selector: &Selector) -> &str {
    match selector {
        Selector::Css(s) => s,
    }
}

#[cfg(test)]
mod tests {
    use crate::config::config::{CrawlConfig, Extractor, Selector, Value};
    use std::convert::TryFrom;

    #[test]
    fn it_works() {
        let config_str = r#"
        {
          "@meta": {
            "version": 1,
            "url": "https://crates.io"
          }
        }
        "#;
        let result = CrawlConfig::try_from(config_str);
        match result {
            Ok(r) => {
                assert_eq!(r.meta.version, 1);
                assert_eq!(r.meta.url, "https://crates.io");
            }
            Err(error) => {
                eprintln!("{}", error);
                panic!();
            }
        }
    }

    #[test]
    fn it_supports_page_with_simple_selectors() {
        let config_str = r#"
        {
          "@meta": {
            "version": 1,
            "url": "https://crates.io"
          },
          "serdeCrate": {
            "@path": "/crates/serde",
            "title": {
              "@selector": {
                "@css": "._heading_87huyj h1"
              },
              "@extract": {
                "@type": "text"
              }
            },
            "titleClassname": {
              "@selector": {
                "@css": "._heading_87huyj"
              },
              "@extract": {
                "@type": "attr",
                "@attr": "class"
              }
            }
          }
        }
        "#;
        let result = CrawlConfig::try_from(config_str);
        match result {
            Ok(r) => {
                let page = r.pages.get("serdeCrate").unwrap();
                let title = page.data.get("title").unwrap();
                let leaf = if let Value::Leaf(result) = title {
                    result
                } else {
                    panic!();
                };
                assert_eq!(r.meta.url, "https://crates.io");
                assert_eq!(page.path, "/crates/serde");
                match &leaf.selector {
                    Selector::Css(slctr) => assert_eq!(slctr, "._heading_87huyj h1"),
                }
                match leaf.extract {
                    Extractor::InnerText => assert!(true),
                    _ => unimplemented!(),
                }
                let title_class = page.data.get("titleClassname").unwrap();
                let leaf = if let Value::Leaf(result) = title_class {
                    result
                } else {
                    panic!();
                };
                match &leaf.selector {
                    Selector::Css(slctr) => assert_eq!(slctr, "._heading_87huyj"),
                }
                match &leaf.extract {
                    Extractor::Attr(attr) => {
                        assert_eq!(attr.name, "class");
                    },
                    _ => unimplemented!(),
                };
            }
            Err(error) => {
                eprintln!("{}", error);
                panic!();
            }
        }
    }
}
