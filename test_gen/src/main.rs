use cfg::ContentNotFoundAction;
use serde_json::Value as Json;
use std::fs::File;
use std::path::PathBuf;
use std::io::{Write, Read};

use crate::prelude::*;
use crate::cfg::Cfg;
use crate::cfg::{JsonPath, JsonPathSeg};

mod prelude;
mod error;
mod cfg;

const CFG_PATH: &str = "cfg.toml";
const INPUT_PATH: &str = "input.json";
const OUTPUT_PATH: &str = "output.txt";

fn write_to_output(buf: &[u8]) -> Result<()> {
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .open(OUTPUT_PATH)?;
    file.write_all(buf)?;
    Ok(())
}

fn get_input_json() -> Result<Json> {
    let mut file = File::options()
        .read(true)
        .open(INPUT_PATH)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let json: Json = serde_json::from_str(&content)?;
    Ok(json)
}

fn get_cfg() -> Result<Cfg> {
    let mut file = File::options()
        .read(true)
        .open(CFG_PATH)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let cfg: Cfg = toml::from_str(&content)?;
    Ok(cfg)
}

#[derive(Debug, Clone, Copy)]
enum Content<'a> {
    Array(&'a Vec<Json>),
    Object(&'a serde_json::Map<String, Json>),
    Else(&'a Json),
}

impl<'a> Content<'a> {
    pub fn from_json(json: &'a Json) -> Self {
        match json {
            Json::Array(a) => Content::Array(a),
            Json::Object(o) => Content::Object(o),
            e => Content::Else(e),
        }
    }

    fn to(&self, path: &JsonPath) -> Option<Self> {
        path.0.iter().fold(Some(*self), |json, pathseg| {
            let Some(json) = json else {
                return None
            };

            match pathseg {
                JsonPathSeg::String(s) => {
                    let Self::Object(object) = self else {
                        return None
                    };
                    let json = object.get(s)?;
                    Some(Content::from_json(json))
                },
                JsonPathSeg::Index(i) => {
                    let Self::Array(array) = self else {
                        return None
                    };
                    let json = array.get(*i)?;
                    Some(Content::from_json(json))
                },
            }
        })
    }
    
    /// Recurisve paths on possible paths to iterate on
    /// Invalid path will be skip
    pub fn iter(&self, paths: &[PathWithPriotiy]) -> ContentIter {
        ContentIter {
            top_content: self,
            possible_path: paths,
            iters: Vec::with_capacity(paths.len()),
            init: true,
        }
    }
}

impl<'a> From<&'a Json> for Content<'a> {
    fn from(json: &'a Json) -> Self {
        Content::from_json(json)
    }
}

struct PathWithPriotiy {
    pub path: JsonPath,
    pub priority: PathPriority,
}

enum PathPriority {
    /// Will return error if not found
    Requried,
    /// Will skip if not found
    Optional,
}

enum ArrayObjectIter<'a> {
    Array(std::slice::Iter<'a, Json>),
    Object(serde_json::map::Iter<'a>),
}

impl<'a> Iterator for ArrayObjectIter<'a> {
    type Item = &'a Json;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ArrayObjectIter::Array(a) => a.next(),
            ArrayObjectIter::Object(o) => o.next().map(|(_, e)| e),
        }
    }
}

enum ContentIterError {
    PathRequiredButNotFound(JsonPath),
}

struct ContentIter<'a> {
    top_content: &'a Content<'a>,
    possible_path: &'a [PathWithPriotiy],
    iters: Vec<Option<ArrayObjectIter<'a>>>,
    init: bool,
}

impl<'a> Iterator for ContentIter<'a> {
    type Item = std::result::Result<Content<'a>, ContentIterError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.init {
            if let Some(iter) = self.iters.last() {
                todo!()
            } else {
                let iter = match self.possible_path.first() {
                    Some(PathWithPriotiy { path, priority }) => match self.top_content.to(path) {
                        Some(content) => todo!(),
                        None => return None,
                    },
                    None => ,
                };
                self.iters.push();
                self.next()
            }
        } else {
            
        }
        
        todo!()
    }
}

fn main() -> Result<()> {
    let cfg: &Cfg = &get_cfg()?; // make it reference so it cannot be manipulated
    let input: Json = get_input_json()?;

    let Cfg {
        func_prefix,
        path_to_cotent,
        content_not_found_action,
    } = cfg;
    
    let content = Content::from_json(&input);
    
    let Some(array) = json_to(&input, path_to_cotent) else {
        return Err(Error::PathToArrayNotExist);
    };

    let Some(array) = array.as_array() else {
        return Err(Error::ItemIsNotArray);
    };
    
    let mut s = String::new();
    for (i, e) in array.iter().enumerate() {
        let Some(e) = json_to(e, path_to_content) else {
            match content_not_found_action {
                ContentNotFoundAction::Skip => continue,
                ContentNotFoundAction::ErrorOut => {
                    return Err(Error::PathToContentNotExist)
                }
            }
        };
        let mut the_func = f!("{func_prefix}{i} => \n");
        let etxt = serde_json::to_string_pretty(e).unwrap();
        let etxt: String = etxt
            .lines()
            .map(|s| f!("    {s}\n"))
            .collect();
        the_func.push_str(&etxt);
        the_func.push('\n');
        s.push_str(&the_func);
    }
    
    write_to_output(s.as_bytes())
}
