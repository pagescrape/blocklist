use std::env;
use std::fs::File;
use std::io::{LineWriter, Write};
use std::path::Path;

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    let response: String = reqwest::Client::new()
        .get("https://blocklistproject.github.io/Lists/alt-version/ads-nl.txt")
        .send()
        .await?
        .text()
        .await?;

    let ads = response.lines().filter(|line| !line.starts_with('#'));

    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file = LineWriter::new(File::create(Path::new(&out_dir).join("blocklist-ads.rs"))?);

    let mut set = phf_codegen::Set::<&'static str>::new();

    for ad in ads.into_iter() {
        set.entry(ad);
    }

    write!(
        &mut file,
        "/// Perfect hash set of advertisement links based on blocklistproject\n"
    )
    .unwrap();
    write!(
        &mut file,
        "pub static BLOCKLIST_ADS_LINKS: phf::Set<&'static str> = {}",
        set.build()
    )
    .unwrap();
    write!(&mut file, ";\n").unwrap();

    file.flush()?;

    Ok(())
}
