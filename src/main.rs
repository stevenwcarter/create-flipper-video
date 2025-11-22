use anyhow::Result;
use clap::Parser;
use regex::Regex;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg()]
    input_dir: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let dir = &args.input_dir;
    let output_path = args.output.clone().unwrap_or_else(|| {
        let folder_name = dir.file_name().unwrap().to_string_lossy();
        dir.parent()
            .unwrap_or_else(|| Path::new("."))
            .join(format!("{}.bmm", folder_name))
    });

    let re = Regex::new(r"^frame_(\d+)\.bm$").unwrap();
    let mut frames: Vec<(u32, PathBuf)> = fs::read_dir(dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let fname = entry.file_name();
            let fname_str = fname.to_string_lossy();
            re.captures(&fname_str)
                .and_then(|cap| cap[1].parse::<u32>().ok().map(|num| (num, entry.path())))
        })
        .collect();

    frames.sort_by_key(|(num, _)| *num);

    let mut output = File::create(output_path)?;
    for (_, path) in frames {
        let mut buf = Vec::new();
        let mut file = File::open(&path)?;
        file.read_to_end(&mut buf)?;
        buf.resize(1025, 0);
        output.write_all(&buf[1..1025])?;
    }

    Ok(())
}
