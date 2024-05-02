use std::ops::Deref;
use zip::unstable::write::FileOptionsExt;
use zip::write::{FileOptionExtension, FileOptions};
use zip::CompressionMethod;

#[derive(Clone)]
pub struct ZipFileOptions<FT: FileOptionExtension>(FileOptions<FT>);

impl<FT: FileOptionExtension> ZipFileOptions<FT> {
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

impl<FT: FileOptionExtension> Deref for ZipFileOptions<FT> {
    type Target = FileOptions<FT>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<FT: FileOptionExtension> FileOptionsExt for ZipFileOptions<FT> {
    fn with_deprecated_encryption(self, password: &[u8]) -> Self {
        Self(self.0.with_deprecated_encryption(password))
    }
}
