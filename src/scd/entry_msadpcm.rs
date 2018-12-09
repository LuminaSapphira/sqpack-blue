use ::FFXIVError;
use super::{SCDEntry, SCDEntryHeader};
use std::io::Write;
use ::byteorder::{WriteBytesExt, LittleEndian};


pub struct SCDEntryMSADPCM {
    header: SCDEntryHeader,
    decoded: Vec<u8>,
}

const WAVE_HEADER_SIZE: usize = 0x10;
const BEGIN_MAGIC: &'static str = "RIFF";
const WAVE_MAGIC: &'static str = "WAVEfmt ";
const DATA_MAGIC: &'static str = "data";

impl SCDEntry for SCDEntryMSADPCM {

    fn create(buffer: &[u8], header: SCDEntryHeader, chunks_offset: &u32, data_offset: &u32, little_end: &bool) -> Result<Box<SCDEntryMSADPCM>, FFXIVError> {
        let final_data_offset = *chunks_offset + header.samples_offset as u32;

        let mut decoded = Vec::<u8>::with_capacity(0x1c + WAVE_HEADER_SIZE + header.data_size as usize);
        decoded.write_all(BEGIN_MAGIC.as_bytes())
            .map_err(|e| FFXIVError::DecodingSCD(Box::new(e)))?;
        let u = 0x14 + WAVE_HEADER_SIZE as i32 + header.data_size;
        decoded.write_i32::<LittleEndian>(u)
            .map_err(|e| FFXIVError::DecodingSCD(Box::new(e)))?;

        decoded.write_all(WAVE_MAGIC.as_bytes())
            .map_err(|e| FFXIVError::DecodingSCD(Box::new(e)))?;

        decoded.write_i32::<LittleEndian>(WAVE_HEADER_SIZE as i32)
            .map_err(|e| FFXIVError::DecodingSCD(Box::new(e)))?;

        decoded.write_all(&buffer[*data_offset as usize..*data_offset as usize + WAVE_HEADER_SIZE])
            .map_err(|e| FFXIVError::DecodingSCD(Box::new(e)))?;

        decoded.write_all(DATA_MAGIC.as_bytes())
            .map_err(|e| FFXIVError::DecodingSCD(Box::new(e)))?;

        decoded.write_i32::<LittleEndian>(header.data_size)
            .map_err(|e| FFXIVError::DecodingSCD(Box::new(e)))?;

        decoded.write_all(&buffer[final_data_offset as usize .. final_data_offset as usize + header.data_size as usize])
            .map_err(|e| FFXIVError::DecodingSCD(Box::new(e)))?;

        Ok(Box::new(SCDEntryMSADPCM {
            header, decoded
        }))

    }

    fn decoded(&self) -> &Vec<u8> {
        &self.decoded
    }

    fn header(&self) -> &SCDEntryHeader {
        &self.header
    }
}