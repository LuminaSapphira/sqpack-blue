extern crate byteorder;
extern crate flate2;

mod index;
mod io;

pub mod hash;
mod expack;
pub mod sheet;


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
    InvalidLanguage(sheet::ex::SheetLanguage, std::collections::HashSet<sheet::ex::SheetLanguage>),
    Custom(String)
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
            InvalidLanguage(req, acc) => write!(f, "The requested language was invalid! Requested: {:?}. Acceptable: {:?}", req, acc),
            Custom(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for FFXIVError {}

impl FFXIV {
    /// Create a new instance of FFXIV to manage access to the data files.
    /// Takes a path to the *sqpack directory*.
    pub fn new(path: &Path) -> Option<FFXIV> {
        if path.exists() {
            Some(FFXIV { path: path.to_path_buf() })
        } else {
            None
        }
    }

    /// Gets a managed file identifier that describes a file within the dat files
    pub fn get_exfile(&self, expath: &String) -> Result<ExFileIdentifier, FFXIVError> {
        ExFileIdentifier::new(expath)
    }

    /// Creates an Index structure from the files on disk that can be later used to
    /// access files.
    pub fn get_index(&self, exfile: &ExFileIdentifier) -> Result<index::Index, FFXIVError> {
        let mut i_file = File::open(
            exfile.get_index_file(self.path.as_path()))
            .map_err(|o| FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
        let ind = io::read_index_file(&mut i_file)?;
        Ok(ind)
    }

    /// Gets the raw data of a file. Determines which index the file would be in,
    /// and returns it as well as the data. get_raw_data_with_index is much faster
    /// and should be preferred as the Index doesn't need to be rebuilt every call.
    pub fn get_raw_data(&self, exfile: &ExFileIdentifier) -> Result<(Vec<u8>, index::Index), FFXIVError> {
        let ind = self.get_index(exfile)?;
        let data = self.get_raw_data_with_index(exfile, &ind)?;
        Ok((data, ind))
    }

    /// Uses a provided index to locate a file in the data files and extract its raw data.
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

    /// Gets the index used for sheets
    pub fn get_sheet_index(&self) -> Result<index::SheetIndex, FFXIVError> {
        let exl_id = self.get_exfile(&String::from("exd/root.exl"))?;
        let index = self.get_index(&exl_id)?;
        Ok(index::SheetIndex::new(index))
    }


    /// Extracts sheet data from the data files. Parses the data into a readable format.
    /// Takes a parameter which is the name of the sheet (without any preceeding exd/
    /// or .exh/exd file extension)
    pub fn get_sheet(&self, exd: &String, language: sheet::ex::SheetLanguage, sheet_index: &index::SheetIndex) -> Result<sheet::Sheet, FFXIVError> {
        use sheet::ex::SheetLanguage;
        let exh_path = String::from(format!("exd/{}.exh", exd.as_str()));
        let exfile = self.get_exfile(&exh_path)?;
        let header = self.get_raw_data_with_index(&exfile, &sheet_index.index)?;
        let info = sheet::decoding::decode_sheet_info(&header)?;
        if !info.languages.contains(&language) { return Err(FFXIVError::InvalidLanguage(language, info.languages)); };
        let mut all_page_data = Vec::<Vec<u8>>::new();
        for page in &info.pages {
            let exd_path = if language == SheetLanguage::None {
                String::from(format!("exd/{}_{}.exd", exd.as_str(), page.page_entry))
            } else {
                String::from(format!("exd/{}_{}_{}.exd", exd.as_str(), page.page_entry, language.get_language_code().unwrap()))
            };
            let page_exfile = self.get_exfile(&exd_path)?;
            let page_data = self.get_raw_data_with_index(&page_exfile, &sheet_index.index)?;
            all_page_data.push(page_data);
        }
        match sheet::decoding::decode_sheet_from_bytes(&info, &all_page_data) {
            Ok(v) => Ok(v),
            Err(e) => Err(e)
        }

    }

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