extern crate byteorder;
extern crate flate2;

mod index;
mod io;

pub mod hash;
mod ex;
mod expack;
mod sheet;

pub use expack::{GameExpansion, FileType, ExFileIdentifier};

mod tests;

//pub use sheet::Sheet;
//use sheet::SheetHeader;

use std::fs::File;
use std::path::{Path,PathBuf};
use std::error::Error;

pub struct FFXIV {
    path: PathBuf,
}

#[derive(Debug)]
pub enum FFXIVError {
    FileNotFound,
    ReadingIndex(Box<std::error::Error>),
    ReadingDat(Box<std::error::Error>),
    DecodingEXD(Box<std::error::Error>),
    DecodingSCD(Box<std::error::Error>),
    MagicMissing,
    UnknownFileType(String),
    UnknownExpansion(String),
    CorruptFileName(String),
}

impl std::fmt::Display for FFXIVError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use FFXIVError::*;
        match self {
            FileNotFound => write!(f, "File not found in index."),
            ReadingIndex(e) => write!(f, "An error occurred while parsing the index file. Inner error: {:?}", e),
            ReadingDat(e) => write!(f, "An error occurred while parsing the dat file. Inner error: {:?}", e),
            DecodingEXD(e) => write!(f, "An error occurred while parsing the EXD file. Inner error: {:?}", e),
            DecodingSCD(e) => write!(f, "An error occurred while parsing the SCD file. Inner error: {:?}", e),
            MagicMissing => write!(f, "The magic marker in a Square Enix file was missing."),
            UnknownFileType(file) => write!(f, "The type of the file was not understood. Requested file: \"{}\"", file),
            UnknownExpansion(file) => write!(f, "The expansion of the file was not understood. Requested file: \"{}\"", file),
            CorruptFileName(file) => write!(f, "Parsing of the file name failed. Requested file: \"{}\"", file),
        }
    }
}

impl std::error::Error for FFXIVError {}

impl FFXIV {
    pub fn new(path: &Path) -> Option<FFXIV> {
        if path.exists() {
            Some(FFXIV { path: path.to_path_buf() })
        } else {
            None
        }
    }

    pub fn get_exfile(&self, expath: &String) -> Result<ExFileIdentifier, FFXIVError> {
        ExFileIdentifier::new(expath)
    }

    pub fn get_index(&self, exfile: &ExFileIdentifier) -> Result<index::Index, FFXIVError> {
        let mut i_file = File::open(
            exfile.get_index_file(self.path.as_path()))
            .map_err(|o| FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
        let ind = io::read_index_file(&mut i_file)?;
        Ok(ind)
    }

    pub fn get_raw_data(&self, exfile: &ExFileIdentifier) -> Result<(Vec<u8>, index::Index), FFXIVError> {
        let ind = self.get_index(exfile)?;
        let data = self.get_raw_data_with_index(exfile, &ind)?;
        Ok((data, ind))
    }

    pub fn get_raw_data_with_index(&self, exfile: &ExFileIdentifier, provided_index: &index::Index) -> Result<Vec<u8>, FFXIVError> {


        let phash = exfile.get_sqpack_hashcode();
        match provided_index.get_file(phash.folder_hash, phash.file_hash) {
            Some(index_file) => {
                let base_dat_path= exfile.get_dat_file(self.path.as_path(), index_file.dat_file);
                let mut dat_file = File::open(
                    base_dat_path.as_path()
                ).map_err(|o| FFXIVError::ReadingDat(Box::<Error>::from(o)))?;
                io::read_data_file(&mut dat_file, &index_file)
            },
            _ => Err(FFXIVError::FileNotFound)
        }

    }

    pub fn get_sheet(&self) {
        unimplemented!();
        // todo
        /*

            parameter: ex path
            1. resolve ex path to .exh file
            2. extract exh file and decode -> ExInfo
            ExInfo {
                Vec<ExLanguages>
                ExLanguages: enum, see https://github.com/viion/ffxiv-datamining/blob/master/research/explorer_exhf_files
                Vec<ExDataType>
                Vec<ExPage>
                ExPage {
                    page_entry: u32 - page number (500 - item_500_en)
                    count: u32 - # of entries in page
                }
            }
            3.

        */
    }


//    pub fn get_sheet(&self, path: &ExPath) -> Result<sheet::Sheet, FFXIVError> {
//        unimplemented!()
//    }
//
//    pub fn get_music(&self, path: &ExPath) -> Result<scd::SCDData, FFXIVError> {
//        unimplemented!()
//    }

    /*

    What I want this to be able to do:
        1. Be able to export raw data files that are not decoded from their FFXIV format
            Need: read dat files
                Need: read index files to find offset in dat files
        2. Decode EXD data sheets into CSV or similar
            Need: 1
            Need: Be able to decode EXH header files in order to parse the data sheet
        3. Decode SCD music files into OGG/WAV (whichever they actually are)
            Need: 1


    */
}