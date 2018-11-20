//use std::collections::HashMap;
use std::fs::File;
use std::error::Error;
use std::io::{SeekFrom, Seek};

use super::byteorder::{ReadBytesExt, LittleEndian};

mod io_dat;
mod io_index;

const SQPACK_MAGIC: u64 = 0x00006B6361507153;


pub fn read_index_file(file: &mut File) -> Result<super::index::Index, ::FFXIVError> {
    let current_pos = file.seek(SeekFrom::Current(0)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;

    assert_eq!(file.read_u64::<LittleEndian>().map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?, SQPACK_MAGIC);

    let h_len = io_index::header_length(file)?;
    let info = io_index::read_index_info(file, h_len.clone())?;
    let sub_folders = io_index::read_directories(file, &info)?;

    file.seek(SeekFrom::Start(current_pos)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;

    Ok(super::index::Index::new(sub_folders))

}

pub fn read_data_file(file: &mut File, file_index: &super::index::File) -> Result<Vec<u8>, ::FFXIVError> {
    let current_pos = file.seek(SeekFrom::Current(0)).map_err(|o| ::FFXIVError::ReadingDat(Box::<Error>::from(o)))?;

    let dat_info = io_dat::read_data_header(file, file_index).map_err(|o| ::FFXIVError::ReadingDat(Box::<Error>::from(o)))?;

    let block_table = io_dat::read_block_table(file, file_index, &dat_info).map_err(|o| ::FFXIVError::ReadingDat(Box::<Error>::from(o)))?;

    /*
     === at Data Entry Header ===
     read u32 - header length
     read u32 - content type, assert type is 0x02.
     read u32 - uncompressed size, store for later comparison assertion
     read u32, dont store (null 4 bytes)
     read u32 - block buffer size. use to initialize later vector with capacity
     read u32 - number of blocks

     === at block table ===
     Loop 0 .. number of blocks {
        load into BlockTableEntry vector (initialized with number of blocks capacity)
            read
     }
    */

    file.seek(SeekFrom::Start(current_pos)).map_err(|o| ::FFXIVError::ReadingDat(Box::<Error>::from(o)))?;

    io_dat::read_and_decompress(file, &dat_info, file_index, &block_table)
}