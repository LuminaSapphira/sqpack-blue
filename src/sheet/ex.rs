
pub struct SheetInfo {
    pub data_types: Vec<SheetDataType>,
    pub pages: Vec<SheetPage>,
    pub languages: Vec<SheetLanguage>,
    pub num_entries: u32
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
            // todo add info to header?
            SheetDataType::String(_) => String::from("str"),
            SheetDataType::Bool(_) => String::from(" u1"),
            SheetDataType::Byte(_) => String::from(" i8"),
            SheetDataType::UByte(_) => String::from(" u8"),
            SheetDataType::Short(_) => String::from("i16"),
            SheetDataType::UShort(_) => String::from("u16"),
            SheetDataType::Int(_) => String::from("i32"),
            SheetDataType::UInt(_) => String::from("u32"),
            SheetDataType::Float(_) => String::from("flt"),
            SheetDataType::PackedInts(_) => String::from("pck"),
            SheetDataType::BitFlags(_) => String::from("bit"),
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
