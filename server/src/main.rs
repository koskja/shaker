#![feature(type_name_of_val)]
mod test;

use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::bail;

fn get_protocol(version: &str) -> anyhow::Result<String> {
    let j = json::parse(&read_to_string("minecraft-data/data/dataPaths.json")?)?;
    let s = j["pc"][version]["protocol"].to_string();
    if s == "null" {
        bail!("Cannot find version {version}")
    }
    let dir = PathBuf::from_str("minecraft-data/data/")?;
    std::fs::read_to_string(dir.join(&s).join("protocol.json")).map_err(Into::into)
}

fn main() -> anyhow::Result<()> {
    let s = get_protocol("1.18")?;
    println!("{s}");
    let _proto = json::parse(&s)?;
    Ok(())
}
