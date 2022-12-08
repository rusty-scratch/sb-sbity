use serde::de::Visitor;
use serde::Deserialize;
use serde_json::Value as Json;
use std::fs::File;
use std::io::{Write, Read};
use std::result::Result as StdResult;

use crate::prelude::*;
use crate::cfg::Cfg;
use crate::path::{Path, Key, PathWithCommands, PathPriority};

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

#[derive(Debug, Clone, Copy)]
enum Content<'a> {
    Array(&'a Json, &'a Vec<Json>),
    Object(&'a Json, &'a serde_json::Map<String, Json>),
    Else(&'a Json)
}

impl<'a> Content<'a> {
    pub fn from_json(json: &'a Json) -> Content<'a> {
        match json {
            Json::Array(a) => Content::Array(json, a),
            Json::Object(o) => Content::Object(json, o),
            e => Content::Else(e),
        }
    }
    
    pub fn json(&self,) -> &'a Json {
        match self {
            Content::Array(j, _) => j,
            Content::Object(j, _) => j,
            Content::Else(j) => j,
        }
     }

    fn to(&self, path: &Path) -> Option<Content<'a>> {
        path.0.iter().fold(Some(*self), |content, pathseg| {
            let Some(content) = content else {
                return None
            };

            match pathseg {
                Key::String(s) => {
                    let Content::Object(_, object) = content else {
                        return None
                    };
                    let next_json = object.get(s)?;
                    Some(Content::from_json(next_json))
                },
                Key::Int(i) => {
                    let Content::Array(_, array) = content else {
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
            Content::Array(_, a) => Some(ArrayObjectIter::Array(a.iter())),
            Content::Object(_, o) => Some(ArrayObjectIter::Object(o.iter())),
            _ => None
        }
    }
    
    /// Recurisve paths on possible paths to iterate on
    /// None when the content is not [`Content::Array`] or [`Content::Object`]
    pub fn iter_paths(&'a self, paths: &'a [PathWithCommands]) -> Option<ContentRecursiveIter<'a>> {
        let mut iter_vec = Vec::with_capacity(paths.len());
        iter_vec.push(self.iter()?);
        Some(ContentRecursiveIter {
            top_content: *self,
            curr_content: *self,
            possible_paths: paths,
            iter_stack: iter_vec,
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
    possible_paths: &'a [PathWithCommands],
    iter_stack: Vec<ArrayObjectIter<'a>>,
}

impl<'a> ContentRecursiveIter<'a> {
    #[inline]
    fn curr_iter(&mut self) -> Option<&mut ArrayObjectIter<'a>> {
        self.iter_stack.last_mut()
    }
    
    fn curr_path(&self) -> Option<&PathWithCommands> {
        let last_idx = self.iter_stack.len().checked_sub(1)?;
        self.possible_paths.get(last_idx)
    }
    
    fn add_stack(&mut self, this: ArrayObjectIter<'a>) {
        self.iter_stack.push(this);
    }
    
    #[inline]
    fn remove_curr_iter(&mut self) {
        self.iter_stack.pop();
    }
    
    #[inline]
    fn curr_iter_next(&mut self) -> Option<Content<'a>> {
        let next_v = self.curr_iter()?.next();
        if next_v.is_none() {
            self.remove_curr_iter();
            return None
        }
        next_v.map(|j| j.into())
    }
    
    #[inline]
    fn is_ended(&self) -> bool {
        // Iters on initialize is guranteed to always have 1 iter
        // So if there's none that means it's done
        self.iter_stack.is_empty()
    }
}

impl<'a> Iterator for ContentRecursiveIter<'a> {
    type Item = StdResult<Content<'a>, ContentIterError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_ended() {
            return None
        }
        
        match self.curr_path() {
            Some(PathWithCommands(path, priority, action)) => {
                let next_content = match self.curr_iter_next() {
                    Some(c) => c,
                    None => match self.next() {
                        Some(Ok(c)) => c,
                        o => return o,
                    },
                };
                let next_content = match next_content.to(path) {
                    Some(n) => n,
                    None => match priority {
                        PathPriority::Requried => Some(Err(ContentIterError::PathRequiredButNotFound(path.clone()))),
                        PathPriority::Optional => todo!(),
                    }
                };
            },
            None => todo!(),
        }
        
        // let resulting_content = match self.curr_path() {
        //     Some(PathWithCommands(path, priority, action)) => {
        //         let next_content = self.curr_content.to(path);
        //         match next_content {
        //             Some(next_content) => Some(Ok(next_content)),
        //             None => match priority {
        //                 PathPriority::Requried => Some(Err(ContentIterError::PathRequiredButNotFound(path.clone()))),
        //                 PathPriority::Optional => self.curr_iter_next(),
        //             },
        //         }
        //     },
        //     None => self.curr_iter_next()
        // };
        // if let Some(Ok(c)) = resulting_content {
        //     self.curr_content = c;
        // }
        // resulting_content
        todo!()
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
        s.push_str(&serde_json::to_string_pretty(&content.json()).unwrap());
        s.push('\n');
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

// #[cfg(test)]
// mod test {
//     use super::Json;
//     use super::Path;
    
//     fn json_to() {
//         let path = Path::from(vec![]);
//     }
// }
