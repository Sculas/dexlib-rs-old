pub use futures::StreamExt; // re-export

use super::class::Class;
use super::Dex;
use super::Result;
use crate::DexReader;
use futures::{stream, Stream};
use std::fs::File;
use std::io::Read;
use std::ops::Index;
use std::path::Path;
use zip::result::ZipError;

type Source = Vec<u8>;

pub struct MultiDex(Vec<Dex<Source>>);

impl MultiDex {
    /// Iterator over the classes
    pub fn classes(&self) -> impl Stream<Item = Class> + '_ {
        stream::iter(&self.0).flat_map(|dex| {
            stream::iter(dex.class_defs()).map(move |classdef| {
                Class::try_from_dex(&dex, &classdef.expect("invalid classdef"))
                    .expect("invalid class")
            })
        })
    }
}

impl Index<usize> for MultiDex {
    type Output = Dex<Source>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

const DEX_EXT: &str = ".dex";

pub struct MultiDexReader;

impl MultiDexReader {
    pub fn from_file<P: AsRef<Path>>(file: P) -> Result<MultiDex> {
        let mut archive = zip::ZipArchive::new(File::open(file.as_ref())?)?;
        let dex_amount = archive
            .file_names()
            .filter(|name| name.ends_with(DEX_EXT))
            .count();
        if dex_amount == 0 {
            return Err(ZipError::UnsupportedArchive("No dex files found").into());
        }
        let mut dexes = Vec::with_capacity(dex_amount);
        for i in 0..archive.len() {
            let mut zf = archive.by_index(i)?;
            if zf.name().ends_with(DEX_EXT) {
                let mut buf = Vec::with_capacity(zf.size() as usize);
                zf.read_to_end(&mut buf)?;
                dexes.push(DexReader::from_vec(buf)?);
            }
        }
        Ok(MultiDex(dexes))
    }
}
