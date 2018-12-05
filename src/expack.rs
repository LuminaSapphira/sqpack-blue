use super::FFXIVError;

use std::path::{Path,PathBuf};
use ::hash::PathHash;
use ::hash;

pub enum FileType {
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
        let lower = expath_str.to_ascii_lowercase();
        let spls: &str = match lower.split("/").next() {
            Some(val) => val,
            _ => return Err(FFXIVError::CorruptFileName(expath_str.clone()))
        };
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
            _ => Err(FFXIVError::UnknownFileType(expath_str.clone()))
        }


    }

    pub fn get_sqpack_code(&self) -> String {
        match self {
            FileType::Common => String::from("00"),
            FileType::BGCommon => String::from("01"),
            FileType::BG => String::from("02"),
            FileType::Cut => String::from("03"),
            FileType::Chara => String::from("04"),
            FileType::Shader => String::from("05"),
            FileType::UI => String::from("06"),
            FileType::Sound => String::from("07"),
            FileType::VFX => String::from("08"),
            FileType::UIScript => String::from("09"),
            FileType::EXD => String::from("0a"),
            FileType::GameScript => String::from("0b"),
            FileType::Music => String::from("0c"),
            FileType::SqpackTest => String::from("12"),
            FileType::Debug => String::from("13"),
        }
    }

    pub fn get_hashcode(&self) -> u8 {
        match self {
            FileType::Common => 0x00,
            FileType::BGCommon => 0x01,
            FileType::BG => 0x02,
            FileType::Cut => 0x03,
            FileType::Chara => 0x04,
            FileType::Shader => 0x05,
            FileType::UI => 0x06,
            FileType::Sound => 0x07,
            FileType::VFX => 0x08,
            FileType::UIScript => 0x09,
            FileType::EXD => 0x0a,
            FileType::GameScript => 0x0b,
            FileType::Music => 0x0c,
            FileType::SqpackTest => 0x12,
            FileType::Debug => 0x13,
        }
    }
}

pub enum GameExpansion {
    FFXIV,
    EX1,
    EX2
}

impl GameExpansion {
    pub fn from_expath_string(expath_str: &String) -> Result<GameExpansion, FFXIVError> {
        let lower = expath_str.to_ascii_lowercase();
        let spls: &str = match lower.split("/").skip(1).next() {
            Some(val) => val,
            _ => return Err(FFXIVError::CorruptFileName(expath_str.clone()))
        };
        match spls {
            "ffxiv" => Ok(GameExpansion::FFXIV),
            "ex1" => Ok(GameExpansion::EX1),
            "ex2" => Ok(GameExpansion::EX2),
            _ => Err(FFXIVError::UnknownExpansion(expath_str.clone()))
        }
    }

    pub fn get_sqpack_code(&self) -> String {
        match self {
            GameExpansion::FFXIV => String::from("00"),
            GameExpansion::EX1 => String::from("01"),
            GameExpansion::EX2 => String::from("02"),
        }
    }

    pub fn get_sqpack_name(&self) -> String {
        match self {
            GameExpansion::FFXIV => String::from("ffxiv"),
            GameExpansion::EX1 => String::from("ex1"),
            GameExpansion::EX2 => String::from("ex2"),
        }
    }

    pub fn get_hashcode(&self) -> u8 {
        match self {
            GameExpansion::FFXIV => 0x00,
            GameExpansion::EX1 => 0x01,
            GameExpansion::EX2 => 0x02,
        }
    }
}

fn parse_number(expath_str: &String) -> Result<u8, FFXIVError> {
    let lower = expath_str.to_ascii_lowercase();
    let spls: &str = match lower.split("/").skip(2).next() {
        Some(val) => val,
        _ => return Err(FFXIVError::CorruptFileName(expath_str.clone()))
    };
    let spls2 = match spls.split("_").next() {
        Some(val) => val,
        _ => return Err(FFXIVError::CorruptFileName(expath_str.clone()))
    };
    if spls2.len() == 3 {
        match u8::from_str_radix(&spls2, 16) {
            Ok(val) => Ok(val),
            _ => Ok(0)
        }
    } else {
        Ok(0)
    }
}

pub struct ExFileIdentifier {

    file_type: FileType,
    expansion: GameExpansion,
    number: u8,
    exfile: String

}

impl ExFileIdentifier {

    pub fn new(expath_str: &String) -> Result<ExFileIdentifier, FFXIVError> {
        if expath_str.len() >= 3 && &expath_str[0..3] == "exd" {
            return Ok(ExFileIdentifier{
                file_type: FileType::EXD,
                expansion: GameExpansion::FFXIV,
                number: 0,
                exfile: expath_str.clone()
            });
        }
        let file_type = FileType::from_expath_string(expath_str)?;
        let expansion = GameExpansion::from_expath_string(expath_str)?;
        let number = parse_number(expath_str)?;
        Ok(ExFileIdentifier{file_type, expansion, number, exfile: expath_str.clone()})
    }

    pub fn get_sqpack_hashcode(&self) -> PathHash {
        hash::compute_path(&self.exfile)
    }

    pub fn get_sqpack_base_file_name(&self) -> String {
        let mut code = String::with_capacity(18);
        code.push_str(self.file_type.get_sqpack_code().as_str());
        code.push_str(self.expansion.get_sqpack_code().as_str());
        code.push_str(format!("{:02x}", self.number).as_str());
        code
    }

    fn get_coded_pathbuf(&self, sqpack_path: &Path) -> PathBuf {
        let code = self.get_sqpack_base_file_name();
        let expbuf = sqpack_path.join(Path::new(&self.expansion.get_sqpack_name()));
        expbuf.join(Path::new(&code))
    }

    pub fn get_dat_file(&self, sqpack_path: &Path, dat_file: u8) -> PathBuf {
        let mut coded_buf = self.get_coded_pathbuf(sqpack_path);
        coded_buf.set_extension(format!("win32.dat{}", dat_file));
        coded_buf
    }
    pub fn get_index_file(&self, sqpack_path: &Path) -> PathBuf {
        let mut coded_buf = self.get_coded_pathbuf(sqpack_path);
        coded_buf.set_extension("win32.index");
        coded_buf
    }
}