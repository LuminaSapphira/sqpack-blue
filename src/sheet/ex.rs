use std::collections::HashSet;

pub struct SheetInfo {
    pub data_types: Vec<SheetDataType>,
    pub pages: Vec<SheetPage>,
    pub languages: HashSet<SheetLanguage>,
    pub num_entries: u32
}

#[derive(Hash, PartialEq, Eq, Debug)]
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

impl std::fmt::Display for SheetLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.get_language_code() {
            Some(val) => write!(f, "[SheetLanguage: {}]", val),
            None => write!(f, "[SheetLanguage: None]"),

        }

    }
}

pub struct SheetPage {
    pub page_entry: u32,
    pub page_size: u32
}

#[derive(Clone, Copy)]
pub enum SheetDataType {
    String(StringInfo),
    Bool(BasicInfo),
    Byte(BasicInfo),
    UByte(BasicInfo),
    Short(BasicInfo),
    UShort(BasicInfo),
    Int(BasicInfo),
    UInt(BasicInfo),
    Float(BasicInfo),
    PackedInts(BasicInfo),
    BitFlags(BitFlagsInfo)
}

impl SheetDataType {
    pub fn get_header(&self) -> String {
        match self {
            SheetDataType::String(_) => String::from("string"),
            SheetDataType::Bool(_) => String::from("bool"),
            SheetDataType::Byte(_) => String::from("int8"),
            SheetDataType::UByte(_) => String::from("uint8"),
            SheetDataType::Short(_) => String::from("int16"),
            SheetDataType::UShort(_) => String::from("uint16"),
            SheetDataType::Int(_) => String::from("int32"),
            SheetDataType::UInt(_) => String::from("uint32"),
            SheetDataType::Float(_) => String::from("float"),
            SheetDataType::PackedInts(_) => String::from("packed"),
            SheetDataType::BitFlags(b_info) => String::from(format!("bitflags[{}]", b_info.bit)),
        }
    }
}

#[derive(Clone, Copy)]
pub struct StringInfo {
    pub strings_offset: u32,
    pub pointer: u16
}

#[derive(Clone, Copy)]
pub struct BitFlagsInfo {
    pub pointer: u16,
    pub bit: u8
}

#[derive(Clone, Copy)]
pub struct BasicInfo {
    pub pointer: u16
}
