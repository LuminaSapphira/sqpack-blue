use std::io::{Seek,SeekFrom};
use std::fs::File;
use std::error::Error;
use super::super::byteorder::{ReadBytesExt, LittleEndian};
use super::super::index;

use std::collections::HashMap;

pub struct IndexInfo {
    pub files_offset: u32,
    pub folders_offset: u32,
    pub files_count: u32,
    pub folders_count: u32
}

/// read header length
pub fn header_length(file: &mut File) -> Result<u32, ::FFXIVError> {

    let current_pos = file.seek(SeekFrom::Current(0)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    file.seek(SeekFrom::Start(0x0c)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    let hlen = file.read_u32::<LittleEndian>().map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    file.seek(SeekFrom::Start(current_pos)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    Ok(hlen)
}

const FILE_INFO_OFFSET: u32 = 0x08;
const FOLDER_INFO_OFFSET: u32 = 0xE4;

/// read index information
pub fn read_index_info(file: &mut File, header_offset: u32) -> Result<IndexInfo, ::FFXIVError> {
    let current_pos = file.seek(SeekFrom::Current(0)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;

    file.seek(SeekFrom::Start(header_offset as u64)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;

    file.seek(SeekFrom::Current(FILE_INFO_OFFSET as i64)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    let f_offset = file.read_u32::<LittleEndian>().map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    let f_len = file.read_u32::<LittleEndian>().map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    let f_count = f_len / 0x10;
    file.seek(SeekFrom::Start(header_offset as u64)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;

    file.seek(SeekFrom::Current(FOLDER_INFO_OFFSET as i64)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    let d_offset = file.read_u32::<LittleEndian>().map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    let d_len = file.read_u32::<LittleEndian>().map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    let d_count = d_len / 0x10;

    file.seek(SeekFrom::Start(current_pos)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    Ok(
        IndexInfo {
            files_offset: f_offset,
            folders_offset: d_offset,
            files_count: f_count,
            folders_count: d_count,
        }
    )
}

pub fn read_file(file: &mut File, offset: u32) -> Result<index::File, ::FFXIVError> {
    let current_pos = file.seek(SeekFrom::Current(0)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;

    file.seek(SeekFrom::Start(offset as u64)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    let file_hash = file.read_u32::<LittleEndian>().map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    let folder_hash = file.read_u32::<LittleEndian>().map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;

    let base_offset = file.read_u32::<LittleEndian>().map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    let dat_file = ((base_offset & 0x7) >> 1) as u8;
    let data_offset = ((base_offset & 0xfffffff8) << 3) as u32;
    file.seek(SeekFrom::Start(current_pos)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;

    Ok(
        index::File {
            file_hash,
            folder_hash,
            data_offset,
            dat_file,
        }
    )
}

pub fn read_folder(file: &mut File, offset: u32) -> Result<index::Folder, ::FFXIVError> {
    let current_pos = file.seek(SeekFrom::Current(0)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;

    file.seek(SeekFrom::Start(offset as u64)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    let folder_hash = file.read_u32::<LittleEndian>().map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    let files_offset = file.read_u32::<LittleEndian>().map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    let files_len = file.read_u32::<LittleEndian>().map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;
    let files_count = files_len / 0x10;

    let mut files = HashMap::<u32, index::File>::new();

    for i in 0..files_count {
        let file = read_file(file, files_offset + i * 0x10)?;
        files.insert(file.file_hash, file);
    }

    file.seek(SeekFrom::Start(current_pos)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;

    Ok(index::Folder::new(folder_hash, files))
}

pub fn read_directories(file: &mut File, index_info: &IndexInfo) -> Result<HashMap<u32, index::Folder>, ::FFXIVError> {
    let current_pos = file.seek(SeekFrom::Current(0)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;

    let mut folders = HashMap::<u32, index::Folder>::new();
    for i in 0..index_info.folders_count {
        let folder = read_folder(file, index_info.folders_offset + i * 0x10)?;
        folders.insert(folder.folder_hash, folder);
    }

    file.seek(SeekFrom::Start(current_pos)).map_err(|o| ::FFXIVError::ReadingIndex(Box::<Error>::from(o)))?;

    Ok(folders)
}