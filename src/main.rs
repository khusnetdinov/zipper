use clap::Parser;
use std::fs::File;
use std::path::Path;
use std::{fs, io};
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

use std::io::{Read, Write};
use std::ops::Deref;
use zip::unstable::write::FileOptionsExt;
use zip::write::FileOptionExtension;

#[derive(Debug, Parser)]
struct Args {
    /// Path to folder
    #[arg(long)]
    path: String,
    /// Password
    #[arg(long)]
    password: String,
}

#[derive(Clone)]
struct FileOptionsExtWrapper<FT: FileOptionExtension>(FileOptions<FT>);

impl<FT: FileOptionExtension> FileOptionsExtWrapper<FT> {
    pub fn new() -> Self {
        Self(FileOptions::default())
    }

    pub fn compression_method(self, method: CompressionMethod) -> Self {
        Self(self.0.compression_method(method))
    }

    pub fn unix_permissions(self, mode: u32) -> Self {
        Self(self.0.unix_permissions(mode))
    }
}

impl<FT: FileOptionExtension> Deref for FileOptionsExtWrapper<FT> {
    type Target = FileOptions<FT>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<FT: FileOptionExtension> FileOptionsExt for FileOptionsExtWrapper<FT> {
    fn with_deprecated_encryption(self, password: &[u8]) -> Self {
        Self(self.0.with_deprecated_encryption(password))
    }
}

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

    let options: FileOptions<()> = *FileOptionsExtWrapper::new()
        .with_deprecated_encryption(password.as_bytes())
        .compression_method(CompressionMethod::Bzip2)
        .unix_permissions(0o755);

    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let file_path = path.unwrap().path();
        let file = File::open(&file_path)?;
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        zip.start_file(file_name, options)?;
        let mut buffer = Vec::new();
        io::copy(&mut file.take(u64::MAX), &mut buffer)?;
        zip.write_all(&buffer)?;
    }

    zip.finish()?;

    println!("Successfully compressed to {}", &zip_file_name);

    Ok(())
}
