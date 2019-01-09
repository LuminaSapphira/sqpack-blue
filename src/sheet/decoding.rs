use super::ex::*;
use super::{Sheet, SheetRow};
use byteorder::{LittleEndian, BigEndian};
use byteorder::ByteOrder;
use ::FFXIVError;

use std::rc::Rc;
use std::collections::HashSet;

/// A magic u32 present at the start of every EXHF File
/// Encodes 'EXHF' in big-endian ASCII
const EXHF_MAGIC: u32 = 0x45584846;
const EXDF_MAGIC: u32 = 0x45584446;


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
        0x0 => {
            Ok(SheetDataType::String(StringInfo{pointer: exh_data_type.1, strings_offset: ds_size.clone() as u32}))
        },
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

fn decode_lang_table(exh_lang_table: &[u8], num_langs: &u16) -> Result<HashSet<SheetLanguage>, FFXIVError> {

    let mut langs = HashSet::<SheetLanguage>::with_capacity(*num_langs as usize);
    for i in 0..*num_langs as usize {
        let lang_code = LittleEndian::read_u16(&exh_lang_table[i * 2 .. i * 2 + 2]);
        langs.insert(
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
        );
    }
    Ok(langs)
}

//fn test_print(hm: &indexmap::IndexMap<usize, u32>) {
//    println!("{{\"array\": [");
//    hm.iter().for_each(|(key, val)| {
//        print!("{{\"key\": {}, \"val\": {}}},", key, val);
//    });
//    println!("]}}");
//}

/// Decodes a sheet from bytes given the header info and all pages of the data file.
pub fn decode_sheet_from_bytes(exh: &SheetInfo, exd: &Vec<Vec<u8>>) -> Result<Sheet, FFXIVError> {

    let types = Rc::new(exh.data_types.to_vec());
    let mut sheet = Sheet {
        rows: indexmap::IndexMap::new(),
        types: types.clone(),
        column_count: exh.data_types.len() as u32
    };

    let mut page_index: usize = 0;
    for page in &exh.pages {
        let pexd: &Vec<u8> = exd.get(page_index).unwrap();
        if pexd.len() < 0x20 {
            return Err(FFXIVError::DecodingEXD(
                Box::new(FFXIVError::Custom(format!("Malformed data in EXDF - length < 0x20")))
            ));
        };

        let magic: u32 = BigEndian::read_u32(&pexd[0..4]);
        if magic != EXDF_MAGIC { return Err(FFXIVError::DecodingEXD(Box::new(FFXIVError::MagicMissing))) };



        let offset_size: u32 = BigEndian::read_u32(&pexd[0x8..0xc]);
        let data_size: u32 = BigEndian::read_u32(&pexd[0xc..0x10]);
        let required_size = 0x20 as usize + offset_size as usize + data_size as usize;
        if pexd.len() < required_size {
            return Err(
                FFXIVError::DecodingEXD(Box::new(
                    FFXIVError::Custom(format!("Malformed data in EXDF. Actual size < Required Size, {} < {}",
                        pexd.len(),
                        required_size
                    ))
                ))
            )
        }

        let offset_start: usize = 0x20;

        let mut exd_table= indexmap::IndexMap::<usize, u32>::with_capacity(page.page_size as usize);
        {
            let mut current_index: usize = 0;
            let mut last_row: Option<usize> = None;
            while last_row.map(|lr| lr < page.page_entry as usize + page.page_size as usize).unwrap_or(true) {
                let r_ind_start = offset_start + 8 * current_index;
                if r_ind_start >= offset_start + offset_size as usize {
                    break;
                }
                let r_ind_end = r_ind_start + 4;
                let r_off_start = r_ind_end.clone();
                let r_off_end = r_off_start + 4;
                let row_index: u32 = BigEndian::read_u32(&pexd[r_ind_start..r_ind_end]);
                let row_offset: u32 = BigEndian::read_u32(&pexd[r_off_start..r_off_end]);

                if exd_table.contains_key(&(row_index as usize)) {
                    return Err(FFXIVError::DecodingEXD(Box::new(FFXIVError::Custom(format!("Duplicate rows in EXDF")))));
                }

                exd_table.insert(row_index as usize, row_offset);
                last_row = Some(row_index as usize);
                current_index += 1;
            }
        }

        for (row_index, row_offset) in exd_table {
            if sheet.rows.contains_key(&(row_index as usize)) {
                return Err(FFXIVError::DecodingEXD(Box::new(FFXIVError::Custom(format!("Duplicate rows in EXDF")))));
            }

            let row_size: u32 = BigEndian::read_u32(&pexd[row_offset as usize .. row_offset as usize + 4]);
            let row_slicer = row_offset as usize + 6;
            let row_slicer_end = row_slicer + row_size as usize;
            if row_slicer_end > pexd.len() {
                return Err(FFXIVError::DecodingEXD(Box::new(FFXIVError::Custom(format!("Malformed Data")))));
            }
            let row_slice: &[u8] = &pexd[row_slicer .. row_slicer_end];

            sheet.rows.insert(row_index as usize, SheetRow {
                types: types.clone(),
                by: row_slice.to_vec()
            });
        }

//        for i in 0..page.page_size as usize {
//
//
//
//        }

        page_index = page_index + 1;
    }

    Ok(sheet)
}

#[cfg(test)]
mod decode_test {
    use super::*;

    #[test]
    fn sheet_header_decode() {
        let exh_path = std::env::var("exh").unwrap();
        use std::fs::File;
        use std::io::Read;
        let mut file_exh = File::open(exh_path).unwrap();
        let mut exh: Vec<u8> = Vec::new();
        file_exh.read_to_end(&mut exh).unwrap();

        let val = decode_sheet_info(&exh).unwrap();
        assert_eq!(val.num_entries, 636);
        assert_eq!(val.pages.get(0).unwrap().page_size, 646);

        val.languages.get(&::sheet::ex::SheetLanguage::None).unwrap();
        match val.data_types.get(6).unwrap() {
            SheetDataType::UByte(d) => assert_eq!(d.pointer, 0x9),
            _ => panic!("incorrect data type")
        };
    }

    #[test]
    fn test_sheet_row_generation() {
        let exd_path = std::env::var("exd").unwrap();
        let exh_path = std::env::var("exh").unwrap();
        use std::fs::File;
        use std::io::Read;
        let mut file_exd = File::open(exd_path).unwrap();
        let mut v: Vec<u8> = Vec::new();
        file_exd.read_to_end(&mut v).unwrap();
        let mut file_exh = File::open(exh_path).unwrap();
        let mut exdv: Vec<u8> = Vec::new();
        file_exh.read_to_end(&mut exdv).unwrap();


        let val = decode_sheet_info(&exdv).unwrap();
        let m = decode_sheet_from_bytes(&val, &vec![v]).unwrap();
        let sr: &SheetRow = m.rows.get(&8).unwrap();
        let title: String = sr.read_cell_data(0).unwrap();
        assert_eq!("music/ffxiv/BGM_Field_Gri_01.scd", title);
    }
}