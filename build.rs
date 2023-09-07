use fst::SetBuilder;
use std::fs::{self, File};
use std::path::Path;
use std::{env, io};

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;

async fn fetch_list(name: &str) -> Result<Vec<String>, GenericError> {
    let response: String = reqwest::Client::new()
        .get(format!(
            "https://blocklistproject.github.io/Lists/alt-version/{name}-nl.txt"
        ))
        .send()
        .await?
        .text()
        .await?;

    let mut list = response
        .lines()
        .filter(|line| !line.starts_with('#'))
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    list.sort();

    build_set_file(name, list.clone())?;

    Ok(list)
}

fn build_set_file(name: &str, list: Vec<String>) -> Result<(), GenericError> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join(format!("blocklist-{name}.fst"));
    let _ = fs::remove_file(&path);
    let file = File::create(path)?;
    let wtr = io::BufWriter::new(file);

    let mut set = SetBuilder::new(wtr)?;

    for item in list.iter() {
        let _ = set.insert(item)?;
    }

    let _ = set.finish()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    let blocked = vec![
        "abuse",
        "drugs",
        "fraud",
        "gambling",
        "malware",
        "phishing",
        "piracy",
        "porn",
        "ransomware",
        "redirect",
        "scam",
        "torrent",
        "tracking",
        "ads",
    ];

    let mut global_list = Vec::new();

    for item in blocked.iter() {
        global_list.append(&mut fetch_list(item).await?);
    }

    global_list.sort();
    global_list.dedup();

    build_set_file("all", global_list)?;

    Ok(())
}
