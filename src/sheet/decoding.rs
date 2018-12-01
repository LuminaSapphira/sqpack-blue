use super::ex::*;
use byteorder::BigEndian;
use ::FFXIVError;

const EXHF_MAGIC: u32 = 0x46485845;

pub fn decode_sheet_info(exh: &Vec<u8>) -> Result<SheetInfo, FFXIVError> {

    let magic: u32 = BigEndian::read_u32(&exh[0..4]);
    if magic != EXHF_MAGIC { return Err(FFXIVError::DecodingEXD(Box::new(FFXIVError::MagicMissing))) };
    let data_type_size: u16 = BigEndian::read_u16(&exh[0x6..0x8]);
    // todo decoding

}

/// assumes the bytes in exd have already been page-concatenated.
pub fn decode_sheet_from_bytes(exh: &Vec<u8>, exd: &Vec<u8>) {

}