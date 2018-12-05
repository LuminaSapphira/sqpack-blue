use super::ex::*;
use super::{Sheet, SheetRow};
use byteorder::BigEndian;
use byteorder::ByteOrder;
use ::FFXIVError;

/// A magic u32 present at the start of every EXHF File
/// Encodes 'EXHF' in big-endian ASCII
const EXHF_MAGIC: u32 = 0x45584846;


/// Decodes a Vec<u8> of the EXHF into a SheetInfo struct
pub fn decode_sheet_info(exh: &Vec<u8>) -> Result<SheetInfo, FFXIVError> {
    if exh.len() < 0x18 {
        return Err(FFXIVError::DecodingEXD(
            Box::new(FFXIVError::Custom("Malformed data in EXHF - length < 0x18".into()))
        ));
    };

    let magic: u32 = BigEndian::read_u32(&exh[0..4]);
    if magic != EXHF_MAGIC { return Err(FFXIVError::DecodingEXD(Box::new(FFXIVError::MagicMissing))) };
    let data_set_size: u16 = BigEndian::read_u16(&exh[0x6..0x8]);
    let num_types: u16 = BigEndian::read_u16(&exh[0x8..0xa]);
    let num_pages: u16 = BigEndian::read_u16(&exh[0xa..0xc]);
    let num_langs: u16 = BigEndian::read_u16(&exh[0xc..0xe]);
    let num_entries: u32 = BigEndian::read_u32(&exh[0x14..0x18]);

    let required_length = 0x20 + (4 * num_types) + (8 * num_pages) + (2 * num_langs);

    if exh.len() < required_length as usize {
        return Err(FFXIVError::DecodingEXD(
            Box::new(FFXIVError::Custom(
                format!("Malformed data in EXHF - number of types, pages, \
                and languages exceeds data size: DataSize({}) < Needed({})", exh.len(), required_length)
                    .into()))
        ));
    };

    let dataset_table_start: usize = 0x20;
    let dataset_table_end: usize = dataset_table_start + 4 * num_types as usize;
    let page_table_start: usize = dataset_table_end.clone();
    let page_table_end: usize = page_table_start + 8 * num_pages as usize;
    let lang_table_start: usize = page_table_end.clone();
    let lang_table_end: usize = lang_table_start + 2 * num_langs as usize;

    let data_types = decode_dataset_definition(
        &exh[dataset_table_start..dataset_table_end], &data_set_size, &num_types)?;
    let pages = decode_page_table(&exh[page_table_start .. page_table_end ], &num_pages);
    let languages = decode_lang_table(&exh[lang_table_start..lang_table_end], &num_langs)?;

    Ok(SheetInfo{
        data_types, languages, pages, num_entries
    })

}

fn decode_dataset_definition(exh_ds_table: &[u8], ds_size: &u16, num_types: &u16) -> Result<Vec<SheetDataType>, FFXIVError> {

    let mut definition = Vec::<SheetDataType>::with_capacity(*num_types as usize);
    for i in 0..*num_types as usize {
        definition.push(decode_data_type(
            (BigEndian::read_u16(&exh_ds_table[i * 4..i * 4 + 2])
             , BigEndian::read_u16(&exh_ds_table[i * 4 + 2..i * 4 + 4])), ds_size,
        )?);
    };
    Ok(definition)

}

fn decode_data_type(exh_data_type: (u16, u16), ds_size: &u16) -> Result<SheetDataType, FFXIVError> {

    match exh_data_type.0 {
        0x0 => Ok(SheetDataType::String(StringInfo{pointer: exh_data_type.1, strings_offset: ds_size.clone() as u32})),
        0x1 => Ok(SheetDataType::Bool(BasicInfo{pointer: exh_data_type.1})),
        0x2 => Ok(SheetDataType::Byte(BasicInfo{pointer: exh_data_type.1})),
        0x3 => Ok(SheetDataType::UByte(BasicInfo{pointer: exh_data_type.1})),
        0x4 => Ok(SheetDataType::Short(BasicInfo{pointer: exh_data_type.1})),
        0x5 => Ok(SheetDataType::UShort(BasicInfo{pointer: exh_data_type.1})),
        0x6 => Ok(SheetDataType::Int(BasicInfo{pointer: exh_data_type.1})),
        0x7 => Ok(SheetDataType::UInt(BasicInfo{pointer: exh_data_type.1})),
        0x9 => Ok(SheetDataType::Float(BasicInfo{pointer: exh_data_type.1})),
        0xb => Ok(SheetDataType::PackedInts(BasicInfo{pointer: exh_data_type.1})),
        d if d >= 0x19 && d < 0x21 => Ok(SheetDataType::BitFlags(BitFlagsInfo {
            pointer: exh_data_type.1, bit: d as u8 - 0x19
        })),
        unknown => Err(FFXIVError::DecodingEXD(Box::new(FFXIVError::Custom(format!("Unknown data type in definition table: {}", unknown)))))
    }

}

fn decode_page_table(exh_page_table: &[u8], num_pages: &u16) -> Vec<SheetPage> {
    let mut pages = Vec::<SheetPage>::with_capacity(*num_pages as usize);
    for i in 0..*num_pages as usize {
        pages.push(SheetPage{
            page_entry: BigEndian::read_u32(&exh_page_table[i * 8 .. i * 8 + 4]),
            page_size: BigEndian::read_u32(&exh_page_table[i * 8 + 4 .. i * 8 + 8])
        });
    }
    pages
}

fn decode_lang_table(exh_lang_table: &[u8], num_langs: &u16) -> Result<Vec<SheetLanguage>, FFXIVError> {
    let mut langs = Vec::<SheetLanguage>::with_capacity(*num_langs as usize);
    for i in 0..*num_langs as usize {
        let lang_code = BigEndian::read_u16(&exh_lang_table[i * 2 .. i * 2 + 2]);
        langs.push(
            match lang_code {
                0x0 => SheetLanguage::None,
                0x1 => SheetLanguage::Japanese,
                0x2 => SheetLanguage::English,
                0x3 => SheetLanguage::German,
                0x4 => SheetLanguage::French,
                0x5 => SheetLanguage::ChineseS,
                0x6 => SheetLanguage::ChineseT,
                0x7 => SheetLanguage::Korean,
                unknown => return Err(FFXIVError::DecodingEXD(
                    Box::new(FFXIVError::Custom(format!("Unknown language code: {}", unknown)))
                ))
            }
        )
    }
    Ok(langs)
}

/// Decodes a sheet from bytes given the header info and the data file.
/// Assumes the bytes in exd have already been page-concatenated.
pub fn decode_sheet_from_bytes(exh: &SheetInfo, exd: &Vec<u8>) -> Sheet {
    use std::rc::Rc;
    let types = Rc::new(exh.data_types.to_vec());
    let mut sheet = Sheet {
        rows: Vec::<SheetRow>::with_capacity(exh.num_entries as usize),
        types: types.clone(),
        column_count: exh.data_types.len() as u32
    };
    for i in 0..5 {
        sheet.rows.push(SheetRow {
            types: types.clone(),
            by: vec![]
        });
    }
    sheet
}

#[cfg(test)]
mod decode_test {
    use super::*;

    #[test]
    fn sheet_header_decode() {
        let exh: Vec<u8> = vec![0x45, 0x58, 0x48, 0x46,
                                0x00, 0x03, 0x00, 0x0C,
                                0x00, 0x07, 0x00, 0x01,
                                0x00, 0x01, 0x00, 0x00,
                                0x00, 0x01, 0x00, 0x00,
                                0x00, 0x00, 0x02, 0x52,
                                0x00, 0x00, 0x00, 0x00,
                                0x00, 0x00, 0x00, 0x00,
                                0x00, 0x00, 0x00, 0x00,
                                0x00, 0x03, 0x00, 0x08,
                                0x00, 0x19, 0x00, 0x0A,
                                0x00, 0x1A, 0x00, 0x0A,
                                0x00, 0x1B, 0x00, 0x0A,
                                0x00, 0x09, 0x00, 0x04,
                                0x00, 0x03, 0x00, 0x09,
                                0x00, 0x00, 0x00, 0x00,
                                0x00, 0x00, 0x02, 0x52,
                                0x00, 0x00];
        let val = decode_sheet_info(&exh).unwrap();
        assert_eq!(val.num_entries, 594);
        assert_eq!(val.pages.get(0).unwrap().page_size, 594);

        match val.languages.get(0).unwrap() {
            SheetLanguage::None => {},
            _ => panic!("incorrect language")
        };
        match val.data_types.get(6).unwrap() {
            SheetDataType::UByte(d) => assert_eq!(d.pointer, 0x9),
            _ => panic!("incorrect data type")
        };
    }

    #[test]
    fn test_sheet_row_generation() {
        let exh: Vec<u8> = vec![0x45, 0x58, 0x48, 0x46,
                                0x00, 0x03, 0x00, 0x0C,
                                0x00, 0x07, 0x00, 0x01,
                                0x00, 0x01, 0x00, 0x00,
                                0x00, 0x01, 0x00, 0x00,
                                0x00, 0x00, 0x02, 0x52,
                                0x00, 0x00, 0x00, 0x00,
                                0x00, 0x00, 0x00, 0x00,
                                0x00, 0x00, 0x00, 0x00,
                                0x00, 0x03, 0x00, 0x08,
                                0x00, 0x19, 0x00, 0x0A,
                                0x00, 0x1A, 0x00, 0x0A,
                                0x00, 0x1B, 0x00, 0x0A,
                                0x00, 0x09, 0x00, 0x04,
                                0x00, 0x03, 0x00, 0x09,
                                0x00, 0x00, 0x00, 0x00,
                                0x00, 0x00, 0x02, 0x52,
                                0x00, 0x00];
        let val = decode_sheet_info(&exh).unwrap();
        let m = decode_sheet_from_bytes(&val, &vec![]);
        let sr: &SheetRow = m.rows.get(2).unwrap();
        let ty: &SheetDataType = sr.types.get(0).unwrap();
        match ty {
            SheetDataType::String(d) => assert_eq!(d.pointer, 0),
            _ => panic!("wrong sheet datas")
        }
    }
}