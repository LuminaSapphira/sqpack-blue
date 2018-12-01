
pub struct SheetInfo {
    pub data_types: Vec<SheetDataType>,
    pub pages: Vec<SheetPage>,
    pub languages: Vec<SheetLanguage>
}

pub enum SheetLanguage {
    None,
    Japanese,
    English,
    German,
    French,
    ChineseS,
    ChineseT,
    Korean
}

impl SheetLanguage {
    pub fn get_language_code(&self) -> Option<String> {
        match self {
            SheetLanguage::None => None,
            SheetLanguage::Japanese => Some(String::from("ja")),
            SheetLanguage::English => Some(String::from("en")),
            SheetLanguage::German => Some(String::from("de")),
            SheetLanguage::French => Some(String::from("fr")),
            SheetLanguage::ChineseS => Some(String::from("chs")),
            SheetLanguage::ChineseT => Some(String::from("cht")),
            SheetLanguage::Korean => Some(String::from("ko")),
        }
    }
}

pub struct SheetPage {
    pub page_entry: u32,
    pub page_size: u32
}

impl From<u16> for SheetLanguage {
    fn from(val: u16) -> SheetLanguage {
        match val {
            0 => SheetLanguage::None,
            1 => SheetLanguage::Japanese,
            2 => SheetLanguage::English,
            3 => SheetLanguage::German,
            4 => SheetLanguage::French,
            5 => SheetLanguage::ChineseS,
            6 => SheetLanguage::ChineseT,
            7 => SheetLanguage::Korean,
        }
    }
}

pub enum SheetDataType {
    UInt(BasicInfo),
    Int(BasicInfo),
    String(StringInfo),
    BitFlags(BitFlagsInfo)
}

impl SheetDataType {
    pub fn get_header(&self) -> String {
        match self {
            // todo add info to header?
            SheetDataType::UInt(_) => String::from("uint32"),
            SheetDataType::Int(_) => String::from("int32"),
            SheetDataType::String(_) => String::from("str"),
            SheetDataType::BitFlags(_) => String::from("bitfl"),
        }
    }
}


pub struct StringInfo {
    pub strings_offset: u32,
    pub pointer: u16
}

pub struct BitFlagsInfo {
    pub pointer: u16,
    pub bit: u8
}

pub struct BasicInfo {
    pub pointer: u16
}
