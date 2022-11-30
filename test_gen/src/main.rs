use serde::de::Visitor;
use serde::Deserialize;
use serde_json::Value as Json;
use std::fs::File;
use std::io::{Write, Read};

use crate::prelude::*;
use crate::cfg::Cfg;
use crate::path::{Path, PathSegment, PathWithPriotiy, PathPriority};

mod path;

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

/// Inside of content is all reference to json
#[derive(Debug, Clone, Copy)]
enum Content<'a> {
    Array(&'a Vec<Json>),
    Object(&'a serde_json::Map<String, Json>),
    Else(&'a Json),
}

impl<'a> Content<'a> {
    pub fn to_cloned_json(&self) -> Json {
        match self {
            Content::Array(a) => Json::Array((*a).clone()),
            Content::Object(o) => Json::Object((*o).clone()),
            Content::Else(j) => (*j).clone(),
        }
    }
    
    pub fn from_json(json: &'a Json) -> Self {
        match json {
            Json::Array(a) => Content::Array(a),
            Json::Object(o) => Content::Object(o),
            e => Content::Else(e),
        }
    }

    fn to(&self, path: &Path) -> Option<Self> {
        path.0.iter().fold(Some(*self), |json, pathseg| {
            let Some(json) = json else {
                return None
            };

            match pathseg {
                PathSegment::String(s) => {
                    let Self::Object(object) = json else {
                        return None
                    };
                    let next_json = object.get(s)?;
                    Some(Content::from_json(next_json))
                },
                PathSegment::Index(i) => {
                    let Self::Array(array) = json else {
                        return None
                    };
                    let next_json = array.get(*i)?;
                    Some(Content::from_json(next_json))
                },
            }
        })
    }
    
    /// None when the content is not [`Content::Array`] or [`Content::Object`]
    pub fn iter(&self) -> Option<ArrayObjectIter<'a>> {
        match self {
            Content::Array(a) => Some(ArrayObjectIter::Array(a.iter())),
            Content::Object(o) => Some(ArrayObjectIter::Object(o.iter())),
            _ => None
        }
    }
    
    /// Recurisve paths on possible paths to iterate on
    /// None when the content is not [`Content::Array`] or [`Content::Object`]
    pub fn iter_paths(&'a self, paths: &'a [PathWithPriotiy]) -> Option<ContentRecursiveIter<'a>> {
        let mut iter_vec = Vec::with_capacity(paths.len());
        iter_vec.push(self.iter()?);
        Some(ContentRecursiveIter {
            top_content: *self,
            curr_content: *self,
            possible_path: paths,
            iters: iter_vec,
            init: true,
        })
    }
}

impl<'a> From<&'a Json> for Content<'a> {
    fn from(json: &'a Json) -> Self {
        Content::from_json(json)
    }
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

#[derive(Debug, thiserror::Error)]
enum ContentIterError {
    #[error("path \"{0}\" is required but not found")]
    PathRequiredButNotFound(Path),
}

struct ContentRecursiveIter<'a> {
    top_content: Content<'a>,
    curr_content: Content<'a>,
    possible_path: &'a [PathWithPriotiy],
    iters: Vec<ArrayObjectIter<'a>>,
    init: bool,
}

impl<'a> Iterator for ContentRecursiveIter<'a> {
    type Item = std::result::Result<Content<'a>, ContentIterError>;

    fn next(&mut self) -> Option<Self::Item> {
        // Iters on initialize is guranteed to always have 1 iter
        // If not that mean it has ended
        let last_idx = self.iters.len().checked_sub(1)?;
        let resulting_content = match self.possible_path.get(last_idx) {
            Some(PathWithPriotiy(path, priority)) => {
                match self.curr_content.to(path) {
                    Some(next_content) => Some(Ok(next_content)),
                    None => match priority {
                        PathPriority::Requried => Some(Err(ContentIterError::PathRequiredButNotFound(path.clone()))),
                        PathPriority::Optional => self.next(),
                    },
                }
            },
            None => self.iters[last_idx].next()
                .map(|j| Ok(j.into())),
        };
        if let Some(Ok(c)) = resulting_content {
            self.curr_content = c;
        }
        resulting_content
    }
}

fn main() -> Result<()> {
    let cfg: &Cfg = &get_cfg()?; // make it reference so it cannot be manipulated
    let input: Json = get_input_json()?;

    let Cfg {
        func_prefix,
        path_to_content,
    } = cfg;
    
    let content = Content::from_json(&input);
    let iter = content.iter_paths(&path_to_content)
        .ok_or(Error::ItemIsNotIteratable)?;
    
    let mut s = String::new();
    for content in iter {
        let content = content?;
        s.push_str(&serde_json::to_string_pretty(&content.to_cloned_json()).unwrap());
        s.push('\n');
    }
    write_to_output(s.as_bytes())
    
    // let Some(array) = json_to(&input, path_to_cotent) else {
    //     return Err(Error::PathToArrayNotExist);
    // };

    // let Some(array) = array.as_array() else {
    //     return Err(Error::ItemIsNotArray);
    // };
    
    // let mut s = String::new();
    // for (i, e) in array.iter().enumerate() {
    //     let Some(e) = json_to(e, path_to_content) else {
    //         match content_not_found_action {
    //             ContentNotFoundAction::Skip => continue,
    //             ContentNotFoundAction::ErrorOut => {
    //                 return Err(Error::PathToContentNotExist)
    //             }
    //         }
    //     };
    //     let mut the_func = f!("{func_prefix}{i} => \n");
    //     let etxt = serde_json::to_string_pretty(e).unwrap();
    //     let etxt: String = etxt
    //         .lines()
    //         .map(|s| f!("    {s}\n"))
    //         .collect();
    //     the_func.push_str(&etxt);
    //     the_func.push('\n');
    //     s.push_str(&the_func);
    // }
    
    // write_to_output(s.as_bytes())
}

#[cfg(test)]
mod test {
    use super::Json;
    use super::Path;
    
    fn json_to() {
        let path = Path::from(vec![]);
    }
}
