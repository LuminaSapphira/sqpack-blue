extern crate byteorder;
extern crate flate2;

mod index;
mod io;
//mod sheet;

#[allow(dead_code)]
pub mod hash;
mod ex;


mod tests;

//pub use sheet::Sheet;
//use sheet::SheetHeader;

use std::fs::File;
use std::path::{Path,PathBuf};

//use byteorder::WriteBytesExt;

//pub fn get_data_offset(file: &mut File) -> u32 {
//
////    let i = io::read_index_file(file);
////    let exd = i.get_file(0xE39B7999, 0xa41d4329)
////        .expect("couldn't unwrap file in lib.rs");
////
////    let exh = i.get_file(0xE39B7999, 0xa0973a01)
////        .expect("couldn't unwrap file in lib.rs");
////
////    let mut dat_file =
////        File::open("C:\\Program Files (x86)\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn\\game\\sqpack\\ffxiv\\0a0000.win32.dat0").expect("not found");
////
////
////    let m = io::read_data_file(&mut dat_file, exd);
////    let mut out_file = File::create("bgm_0.exd").unwrap();
////    for by in m {
////        out_file.write_u8(by).unwrap();
////    }
////    let m2 = io::read_data_file(&mut dat_file, exh);
////    let mut out_file2 = File::create("bgm.exh").unwrap();
////    for by2 in m2 {
////        out_file2.write_u8(by2).unwrap();
////    }
//
//    3
//}

enum FileType {
    Common,
    BGCommon,
    BG,
    Cut,
    Chara,
    Shader,
    UI,
    Sound,
    VFX,
    UIScript,
    EXD,
    GameScript,
    Music,
    SqpackTest,
    Debug,
}

impl FileType {
    pub fn from_expath_string(expath_str: &String) -> Result<FileType, FFXIVError> {

        let lower = val.to_ascii_lowercase();
        let spls: &str = lower.split("/").next();
        match spls {
            "common" => Ok(FileType::Common),
            "bgcommon" => Ok(FileType::BGCommon),
            "bg" => Ok(FileType::BG),
            "cut" => Ok(FileType::Cut),
            "chara" => Ok(FileType::Chara),
            "shader" => Ok(FileType::Shader),
            "ui" => Ok(FileType::UI),
            "sound" => Ok(FileType::Sound),
            "vfx" => Ok(FileType::VFX),
            "ui_script" => Ok(FileType::UIScript),
            "exd" => Ok(FileType::EXD),
            "game_script" => Ok(FileType::GameScript),
            "music" => Ok(FileType::Music),
            "_sqpack_test" => Ok(FileType::SqpackTest),
            "_debug" => Ok(FileType::Debug),
        }


    }

    pub fn get_sqpack_code(&self) -> String {
        
        use FileType::*;
        match self {
            Common => String::from("") // file name should be 0?0000.{index,dat} where the ? corresponds to file type
            // but, in the actual files, the 4 zeroes afterwards also seem to mean something, but I do not know what.
            // more research will be necessary
        }
    }
}

#[allow(dead_code)]
enum GameExpansion {
    FFXIV,
    EX1,
    EX2
}

pub struct ExPath {
    file_type: u8,
    expansion: GameExpansion,

}

#[allow(dead_code)]
pub struct FFXIV {
    path: PathBuf
}

#[derive(Debug)]
pub enum FFXIVError {
    FileNotFound,
    ReadingIndex(Box<std::error::Error>),
    ReadingDat(Box<std::error::Error>),
    DecodingEXD(Box<std::error::Error>),
    DecodingSCD(Box<std::error::Error>),
    MagicMissing,
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
        }
    }
}

impl std::error::Error for FFXIVError {}

impl FFXIV {
    pub fn new(path: &Path) -> Option<FFXIV> {
        if path.exists() {
            Some(FFXIV {path: path.to_path_buf()})
        }
        else {
            None
        }
    }

    pub fn get_expath(&self, exfile: &String) -> Result<ExPath, FFXIVError> {
        unimplemented!()
        //TODO impl
    }

    pub fn get_raw_data(&self, path: &ExPath) -> Result<Vec<u8>, FFXIVError> {
        /*
        let mut i_file: File = File::open(path.get_index_file());

        let index: Index = io::read_index_file(&mut i_file);
        let dat_loc =
        let mut d_file: File = File::open(path.get_dat_file(dat_loc));
        */
        Err(FFXIVError::FileNotFound)
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

//struct GameData {
////    pub path: Path
//}
//
//struct GamePath {
//    pub folder_hash: u32,
//    pub file_hash: u32,
//    pub expansion: GameExpansion
//}
//
//impl GameData {
//    pub fn get_raw_data(&self, location: &GamePath) -> Vec<u8> {
//        unimplemented!();
//    }
//    pub fn get_sheet(&self, exd_path: &String, exd_name: &String) -> Sheet {
//        let mut exh_path = exd_path.clone();
//        exh_path.insert_str(0, "exd/");
//        let mut exh_name = exd_name.clone();
//        exh_name.push_str(".exh");
//        let gp = GamePath {
//            folder_hash: hash::compute(&exh_path),
//            file_hash: hash::compute(&exh_name),
//            expansion: GameExpansion::FFXIV
//        };
//        unimplemented!();
//    }
//}
//
//impl GamePath {
//    pub fn new(path: &String, file: &String) -> GamePath {
//        unimplemented!();
//    }
//}
//
//pub fn prepare_game_data(path: &std::path::Path) -> GameData {
//    GameData { path: path.clone() }
//}

/*


 impl GameExpansion {

 }
 struct GameData {
   path: std::path::Path
 }
 impl GameData {
   pub fn get_raw_data(&self, location: &GamePath, ) -> Vec<u8> {}

   // exd path is after /exd/ that all share, exd name is without extension
   // auto-resolves to "${exd_name}.exh" internally
   pub fn get_sheet(&self, exd_path: &String, exd_name: &String) -> Sheet {}
 }
 struct GamePath {
   folder_hash: u32,
   file_hash: u32,
   expansion: u8
 }

 pub fn prepare_game_data(path: std::path::Path) -> GameData {}



 pub fn get_path(path: &String, file: &String) -> GamePath {}


 pub fn get_sheet(exd_path: &String, exd_name: &String) -> Sheet {}



*/