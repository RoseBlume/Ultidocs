use std::collections::HashMap;
use std::error::Error;
use std::fs;

use ultijson::{parse, JsonValue};

#[derive(Clone)]
pub struct Config {
    pub title: String,
    pub site_root: String,
    pub favicon: Option<String>,
    pub content_dir: String,
    pub build_dir: String,
    pub custom_css: Option<String>,
    pub sidebar_css: Option<String>,
    pub highlight_css: Option<String>,
    pub sidebar: Vec<SidebarItem>,
}

#[derive(Clone)]
pub struct SidebarItem {
    pub label: String,
    pub slug: Option<String>,
    pub collapsed: Option<bool>,
    pub items: Option<Vec<SidebarItem>>,
    pub autogenerate: Option<AutoGenerate>,
}

#[derive(Clone)]
pub struct AutoGenerate {
    pub directory: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let json = parse(&content)?;
        let obj = match json {
            JsonValue::Object(map) => map,
            _ => return Err("Config must be a JSON object".into()),
        };

        Ok(Self {
            title: Self::get_string(&obj, "title")?,
            site_root: Self::get_string(&obj, "site_root")?,
            content_dir: Self::get_string(&obj, "content_dir")?,
            build_dir: Self::get_string(&obj, "build_dir")?,
            favicon: Self::get_optional_string(&obj, "favicon"),
            custom_css: Self::get_optional_string(&obj, "custom_css"),
            sidebar_css: Self::get_optional_string(&obj, "sidebar_css"),
            highlight_css: Self::get_optional_string(&obj, "highlight_css"),           
            sidebar: SidebarItem::parse_sidebar(&obj)?,
        })
    }

    fn get_string(
        map: &HashMap<String, JsonValue>,
        key: &str,
    ) -> Result<String, Box<dyn Error>> {
        match map.get(key) {
            Some(JsonValue::String(s)) => Ok(s.clone()),
            _ => Err(format!("Missing or invalid {}", key).into()),
        }
    }

    fn get_optional_string(
        map: &HashMap<String, JsonValue>,
        key: &str,
    ) -> Option<String> {
        match map.get(key) {
            Some(JsonValue::String(s)) => Some(s.clone()),
            _ => None,
        }
    }
}

impl SidebarItem {
    fn parse_sidebar(
        obj: &HashMap<String, JsonValue>,
    ) -> Result<Vec<SidebarItem>, Box<dyn Error>> {
        let arr = match obj.get("sidebar") {
            Some(JsonValue::Array(a)) => a,
            _ => return Err("sidebar must be array".into()),
        };

        arr.iter().map(Self::from_json).collect()
    }

    fn from_json(val: &JsonValue) -> Result<SidebarItem, Box<dyn Error>> {
        let obj = match val {
            JsonValue::Object(map) => map,
            _ => return Err("Invalid sidebar item".into()),
        };

        Ok(SidebarItem {
            label: Config::get_string(obj, "label")?,
            slug: Self::get_optional_string(obj, "slug"),
            collapsed: Self::get_optional_bool(obj, "collapsed"),
            items: match obj.get("items") {
                Some(JsonValue::Array(arr)) => {
                    Some(arr.iter().map(Self::from_json).collect::<Result<_, _>>()?)
                }
                _ => None,
            },
            autogenerate: match obj.get("autogenerate") {
                Some(JsonValue::Object(map)) => {
                    Some(AutoGenerate {
                        directory: Config::get_string(map, "directory")?,
                    })
                }
                _ => None,
            },
        })
    }

    fn get_optional_string(
        map: &HashMap<String, JsonValue>,
        key: &str,
    ) -> Option<String> {
        match map.get(key) {
            Some(JsonValue::String(s)) => Some(s.clone()),
            _ => None,
        }
    }

    fn get_optional_bool(
        map: &HashMap<String, JsonValue>,
        key: &str,
    ) -> Option<bool> {
        match map.get(key) {
            Some(JsonValue::Bool(b)) => Some(*b),
            _ => None,
        }
    }
}