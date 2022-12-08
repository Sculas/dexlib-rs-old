use super::class::Class;
use super::Dex;
use super::Result;
use crate::class::ClassDefItem;
use crate::DexReader;
use std::fs::File;
use std::io::Read;
use std::ops::Index;
use std::path::Path;
use zip::result::ZipError;

type Source = Vec<u8>;

macro_rules! for_all_dexes {
    ($self:ident, $func:ident, $e:expr$(, $extra:ident)*) => {
        $self.0.iter().$func($e)$(.$extra())*
    };
}

pub struct MultiDex(Vec<Dex<Source>>);

impl MultiDex {
    /// Iterate over the classes ([`Class`]) contained in all dex files.
    pub fn classes(&self) -> impl Iterator<Item = Class> + '_ {
        for_all_dexes!(self, flat_map, |dex| {
            dex.class_defs().map(move |classdef| {
                Class::try_from_dex(&dex, &classdef.expect("invalid classdef"))
                    .expect("invalid class")
            })
        })
    }

    /// Iterate over the raw class definitions ([`ClassDefItem`]) contained in all dex files.
    pub fn class_defs(&self) -> impl Iterator<Item = ClassDefItem> + '_ {
        for_all_dexes!(self, flat_map, |dex| {
            dex.class_defs().map(|x| x.expect("invalid classdef"))
        })
    }

    /// Returns the amount of classes ([`Class`]) contained in all dex files.
    pub fn classes_amount(&self) -> u32 {
        for_all_dexes!(self, map, |dex| dex.class_defs_amount(), sum)
    }

    /// Finds a [`Class`] by the given class name. The name should be in smali format.
    pub fn class_by_name(&self, descriptor: &str) -> Option<Class> {
        for_all_dexes!(self, find_map, |dex| dex
            .find_class_by_name(descriptor)
            .expect("invalid classdef"))
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
