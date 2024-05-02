use std::ops::Deref;
use zip::unstable::write::FileOptionsExt;
use zip::write::{FileOptionExtension, FileOptions};
use zip::CompressionMethod;

#[derive(Clone)]
pub struct ZipFileOptions<FOT: FileOptionExtension>(FileOptions<FOT>);

impl<FOT: FileOptionExtension> ZipFileOptions<FOT> {
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

impl<FOT: FileOptionExtension> Deref for ZipFileOptions<FOT> {
    type Target = FileOptions<FOT>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<FOT: FileOptionExtension> FileOptionsExt for ZipFileOptions<FOT> {
    fn with_deprecated_encryption(self, password: &[u8]) -> Self {
        Self(self.0.with_deprecated_encryption(password))
    }
}
