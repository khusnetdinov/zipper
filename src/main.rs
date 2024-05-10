mod args;
mod zip_file_options;

use crate::zip_file_options::ZipFileOptions;
use args::Args;
use byte_unit::Byte;
use clap::Parser;
use splitfile::SplitFile;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::{fs, io};
use zip::unstable::write::FileOptionsExt;
use zip::{CompressionMethod, ZipWriter};

fn write(
    mut zip: ZipWriter<File>,
    options: ZipFileOptions<()>,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Start compressing {}", &path.display());

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

    println!("Successfully compressed {}", &path.display());

    Ok(())
}

fn chunk(path: &Path, size: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut zip_file = File::open(path)?;
    let volsize = Byte::parse_str(size, true).unwrap();

    println!(
        "Start chinking {} with size {} to chinks sized {}",
        &path.display(),
        &zip_file.metadata().unwrap().len(),
        volsize
    );

    let chunk_path_string = match path.parent() {
        Some(parent) => format!(
            "{}/chunked-{}",
            parent.display(),
            &path.file_name().unwrap().to_str().unwrap()
        ),
        _ => format!("chunked-{}", &path.file_name().unwrap().to_str().unwrap()),
    };

    let chunk_path = Path::new(&chunk_path_string);
    let mut split_file = SplitFile::create(chunk_path, volsize.into())?;
    let mut zip_data = Vec::new();

    zip_file.read_to_end(&mut zip_data)?;
    split_file.write(&zip_data).expect("write error");

    println!("Successfully chunking {}", &path.display());

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args {
        path,
        password,
        size,
        ..
    } = Args::parse();
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
    let zip = ZipWriter::new(zip_file);

    let options: ZipFileOptions<()> = ZipFileOptions::new()
        .with_deprecated_encryption(password.as_bytes())
        .compression_method(CompressionMethod::DEFLATE)
        .unix_permissions(0o755);

    write(zip, options, path)?;
    chunk(zip_file_path, size)?;

    Ok(())
}
