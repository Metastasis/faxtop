use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    // pub value: Option<Value>,
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
pub struct Pages {
    #[serde(rename(deserialize = "@path"))]
    pub path: String,
    #[serde(flatten)]
    pub data: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CrawlConfig {
    #[serde(rename(deserialize = "@meta"))]
    pub meta: Meta,
    #[serde(flatten)]
    pub pages: HashMap<String, Pages>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub version: u8,
    pub url: String,
}
