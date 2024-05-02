mod args;
mod zip_file_options;

use crate::zip_file_options::ZipFileOptions;
use args::Args;
use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::{fs, io};
use zip::unstable::write::FileOptionsExt;
use zip::{CompressionMethod, ZipWriter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args { path, password, .. } = Args::parse();
    let path = Path::new(&path);

    if !path.exists() {
        panic!("Path \"{}\" is doesn't exists", &path.display());
    }

    let zip_file_name = format!("{}.zip", &path.display());
    let zip_file_path = Path::new(&zip_file_name);

    if zip_file_path.exists() {
        fs::remove_file(&zip_file_name)?;
    };

    let zip_file = File::create(&zip_file_name)?;
    let mut zip = ZipWriter::new(zip_file);

    let options: ZipFileOptions<()> = ZipFileOptions::new()
        .with_deprecated_encryption(password.as_bytes())
        .compression_method(CompressionMethod::DEFLATE)
        .unix_permissions(0o755);

    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let file_path = path.unwrap().path();
        let file = File::open(&file_path)?;
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        zip.start_file(file_name, *options)?;
        let mut buffer = Vec::new();
        io::copy(&mut file.take(u64::MAX), &mut buffer)?;
        zip.write_all(&buffer)?;
    }

    zip.finish()?;

    println!("Successfully compressed to {}", &zip_file_name);

    Ok(())
}
