use ::byteorder::{BigEndian, LittleEndian, ByteOrder};

use ::FFXIVError;

const VERSION: std::ops::Range<usize> = 0x8..0x8 + 4 as usize;

use super::SCDHeader;
use super::entry::*;
use super::entry_msadpcm::*;
use super::entry_ogg::*;

pub fn decode_little_endianness(buffer: &[u8]) -> Result<bool, FFXIVError> {

    if buffer.len() < (VERSION.end) as usize {
        return Err(FFXIVError::DecodingSCD(
            Box::new(FFXIVError::Custom(format!("Buffer too short."))))
        )
    }
    let ver_big_endian = BigEndian::read_u32(&buffer[VERSION]);
    let ver_little_endian = LittleEndian::read_u32(&buffer[VERSION]);
    if ver_big_endian == 2 || ver_big_endian == 3 {
        Ok(false)
    } else if ver_little_endian == 2 || ver_little_endian == 3 {
        Ok(true)
    } else {
        Err(FFXIVError::DecodingSCD(
            Box::new(FFXIVError::Custom(format!("Unable to determine endianness.")))
        ))
    }
}

pub fn decode_scd_header(buffer: &[u8], little_end: &bool) -> Result<SCDHeader, FFXIVError> {
    let file_header_size = read_i16(&0xe, buffer, little_end)? as usize;
    Ok(
        SCDHeader {
            unknown_1_count: read_i16(&(file_header_size as usize), buffer, little_end)?,
            unknown_2_count: read_i16(&(file_header_size + 0x2 as usize), buffer, little_end)?,
            entry_count: read_i16(&(file_header_size + 0x4 as usize), buffer, little_end)?,
            unknown_1: read_i16(&(file_header_size + 0x6 as usize), buffer, little_end)?,
            unknown_1_offset: read_i32(&(file_header_size + 0x8 as usize), buffer, little_end)?,
            entry_table_offset: read_i32(&(file_header_size + 0xc as usize), buffer, little_end)?,
            unknown_2_offset: read_i32(&(file_header_size + 0x10 as usize), buffer, little_end)?,
            unknown_2: read_i32(&(file_header_size + 0x14 as usize), buffer, little_end)?,
            unknown_offset_1: read_i32(&(file_header_size + 0x18 as usize), buffer, little_end)?,
        }
    )
}

pub fn decode_scd_entries(buffer: &[u8], scd_header: &SCDHeader, little_end: &bool) -> Result<Vec<Box<SCDEntry>>, FFXIVError> {
    let mut scd_entry_headers = Vec::<SCDEntryHeader>::with_capacity(scd_header.entry_count as usize);
    let mut entry_chunk_offsets = Vec::<u32>::with_capacity(scd_entry_headers.len());
    let mut entry_data_offsets = Vec::<u32>::with_capacity(scd_entry_headers.len());
    for i in 0..scd_header.entry_count as usize {
        let header_offset = read_i32(&((scd_header.entry_table_offset as usize + 4 * i) as usize), buffer, little_end)?;
        scd_entry_headers.push(decode_entry_header(&(header_offset as usize), buffer, little_end)?);
        entry_chunk_offsets.push(header_offset as u32 + 32);
        entry_data_offsets.push(entry_chunk_offsets[i].clone());
        for j in 0..scd_entry_headers[i].aux_chunk_count {
            entry_data_offsets[i] = entry_data_offsets[i] +
                read_i32(&(entry_data_offsets[i] as usize + 4), buffer, little_end)? as u32;
        };
    };

    let mut entries = Vec::<Box<SCDEntry>>::with_capacity(scd_header.entry_count as usize);
    println!("{:?}", scd_entry_headers);
    for i in 0..scd_header.entry_count as usize {
        let pop = match scd_entry_headers.pop() {
            Some(val) => val,
            None => return Err(FFXIVError::DecodingSCD(Box::new(FFXIVError::Custom(format!("Unknown issue with entry headers."))))),
        };
        entries.push(decode_entry(buffer, pop, &entry_chunk_offsets[i], &entry_data_offsets[i], little_end)?);
    };
    Ok(entries)
}

fn decode_entry_header(offset: &usize, buffer: &[u8], little_end: &bool) -> Result<SCDEntryHeader, FFXIVError> {
    let codec = match read_i32(&(offset + 0xc), buffer, little_end)? {
        0x0 => SCDCodec::None,
        0x6 => SCDCodec::OGG,
        0x0C => SCDCodec::MSADPCM,
        u => return Err(FFXIVError::DecodingSCD(Box::new(FFXIVError::Custom(format!("Unknown SCD codec: {}", u))))),
    };
    Ok(
        SCDEntryHeader {
            data_size: read_i32(&(offset), buffer, little_end)?,
            channel_count: read_i32(&(offset+0x4), buffer, little_end)?,
            frequency: read_i32(&(offset+0x8), buffer, little_end)?,
            codec,
            loop_start: read_i32(&(offset+0x10), buffer, little_end)?,
            loop_end: read_i32(&(offset+0x14), buffer, little_end)?,
            samples_offset: read_i32(&(offset+0x18), buffer, little_end)?,
            aux_chunk_count: read_i16(&(offset+0x1c), buffer, little_end)?,
            unknown_1: read_i16(&(offset+0x1e), buffer, little_end)?,
        }
    )
}

fn decode_entry(buffer: &[u8], entry_header: SCDEntryHeader, entry_chunk_offset: &u32, entry_data_offset: &u32, little_end: &bool) -> Result<Box<SCDEntry>, FFXIVError> {
    if entry_header.data_size == 0 {
        Ok(SCDEntryNone::create(buffer, entry_header, entry_chunk_offset, entry_data_offset, little_end)?)
    } else {
        match entry_header.codec {
            SCDCodec::OGG => Ok(SCDEntryOgg::create(buffer, entry_header, entry_chunk_offset, entry_data_offset, little_end)?),
            SCDCodec::MSADPCM => Ok(SCDEntryMSADPCM::create(buffer, entry_header, entry_chunk_offset, entry_data_offset, little_end)?),
            SCDCodec::None => Ok(SCDEntryNone::create(buffer, entry_header, entry_chunk_offset, entry_data_offset, little_end)?),
            _ => Err(FFXIVError::DecodingSCD(Box::new(FFXIVError::Custom(format!("Unknown codec!")))))
        }
    }
}

fn check_len(offset: &usize, buffer: &[u8], size: usize) -> Result<(), FFXIVError> {
    if buffer.len() < offset + size {
        Err(FFXIVError::DecodingSCD(
            Box::new(FFXIVError::Custom(
                format!("Buffer too short. Length: {}, Requested: {}", buffer.len(), offset + 2)))
            )
        )
    } else {
        Ok(())
    }
}

pub fn read_i16(offset: &usize, buffer: &[u8], little_end: &bool) -> Result<i16, FFXIVError> {
    check_len(offset, buffer, 2)?;
    if *little_end {
        Ok(LittleEndian::read_i16(&buffer[*offset..*offset + 2]))
    } else {
        Ok(BigEndian::read_i16(&buffer[*offset..*offset + 2]))
    }
}

pub fn read_i32(offset: &usize, buffer: &[u8], little_end: &bool) -> Result<i32, FFXIVError> {
    check_len(offset, buffer, 4)?;
    if *little_end {
        Ok(LittleEndian::read_i32(&buffer[*offset..*offset + 4]))
    } else {
        Ok(BigEndian::read_i32(&buffer[*offset..*offset + 4]))
    }
}

pub fn read_i64(offset: &usize, buffer: &[u8], little_end: &bool) -> Result<i64, FFXIVError> {
    check_len(offset, buffer, 8)?;
    if *little_end {
        Ok(LittleEndian::read_i64(&buffer[*offset..*offset + 8]))
    } else {
        Ok(BigEndian::read_i64(&buffer[*offset..*offset + 8]))
    }
}