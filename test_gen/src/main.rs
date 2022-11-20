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

fn json_to<'a>(json: &'a Json, path: &JsonPath) -> Option<&'a Json> {
    path.0.iter().fold(Some(json), |json, pathseg| {
        let Some(json) = json else {
            return None
        };

        match pathseg {
            JsonPathSeg::String(s) => {
                let Some(object) = json.as_object() else {
                    return None
                };
                object.get(s)
            },
            JsonPathSeg::Index(i) => {
                let Some(array) = json.as_array() else {
                    return None
                };
                array.get(*i)
            },
        }
    })
}

fn main() -> Result<()> {
    let cfg: &Cfg = &get_cfg()?; // make it reference so it cannot be manipulated
    let input: Json = get_input_json()?;

    let Cfg {
        func_prefix,
        path_to_array,
        path_to_content,
        content_not_found_action,
    } = cfg;
    
    let Some(array) = json_to(&input, path_to_array) else {
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
