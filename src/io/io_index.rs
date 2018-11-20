use std::io::{Seek,SeekFrom};
use std::fs::File;
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
pub fn header_length(file: &mut File) -> u32 {

    let current_pos = file.seek(SeekFrom::Current(0)).unwrap();
    file.seek(SeekFrom::Start(0x0c)).unwrap();
    let hlen = file.read_u32::<LittleEndian>().unwrap();
    file.seek(SeekFrom::Start(current_pos)).unwrap();
    hlen
}

const FILE_INFO_OFFSET: u32 = 0x08;
const FOLDER_INFO_OFFSET: u32 = 0xE4;

/// read index information
pub fn read_index_info(file: &mut File, header_offset: u32) -> IndexInfo {
    let current_pos = file.seek(SeekFrom::Current(0)).unwrap();

    file.seek(SeekFrom::Start(header_offset as u64)).unwrap();

    file.seek(SeekFrom::Current(FILE_INFO_OFFSET as i64)).unwrap();
    let f_offset = file.read_u32::<LittleEndian>().unwrap();
    let f_len = file.read_u32::<LittleEndian>().unwrap();
    let f_count = f_len / 0x10;
    file.seek(SeekFrom::Start(header_offset as u64)).unwrap();

    file.seek(SeekFrom::Current(FOLDER_INFO_OFFSET as i64)).unwrap();
    let d_offset = file.read_u32::<LittleEndian>().unwrap();
    let d_len = file.read_u32::<LittleEndian>().unwrap();
    let d_count = d_len / 0x10;

    file.seek(SeekFrom::Start(current_pos)).unwrap();

    IndexInfo {
        files_offset: f_offset,
        folders_offset: d_offset,
        files_count: f_count,
        folders_count: d_count
    }
}

pub fn read_file(file: &mut File, offset: u32) -> index::File {
    let current_pos = file.seek(SeekFrom::Current(0)).expect(format!("failed to get stream pos, offset: {}", offset).as_str());

    file.seek(SeekFrom::Start(offset as u64)).expect(format!("failed to seek stream pos, offset: {}", offset).as_str());
    let file_hash = file.read_u32::<LittleEndian>().expect(format!("failed to get file hash, offset: {}", offset).as_str());
    let folder_hash = file.read_u32::<LittleEndian>().expect(format!("failed to get folder hash, offset: {}", offset).as_str());

    let base_offset = file.read_u32::<LittleEndian>().expect(format!("failed to get data offset, offset: {}", offset).as_str());
    let dat_file = ((base_offset & 0x7) >> 1) as u8;
    let data_offset = ((base_offset & 0xfffffff8) << 3) as u32;
    file.seek(SeekFrom::Start(current_pos)).expect(format!("failed to return to start, offset: {}", offset).as_str());

    index::File {
        file_hash,
        folder_hash,
        data_offset,
        dat_file
    }
}

pub fn read_folder(file: &mut File, offset: u32) -> index::Folder {
    let current_pos = file.seek(SeekFrom::Current(0)).unwrap();

    file.seek(SeekFrom::Start(offset as u64)).unwrap();
    let folder_hash = file.read_u32::<LittleEndian>().unwrap();
    let files_offset = file.read_u32::<LittleEndian>().unwrap();
    let files_len = file.read_u32::<LittleEndian>().unwrap();
    let files_count = files_len / 0x10;

    let mut files = HashMap::<u32, index::File>::new();

    for i in 0..files_count {
        let file = read_file(file, files_offset + i * 0x10);
        files.insert(file.file_hash, file);
    }

    file.seek(SeekFrom::Start(current_pos)).unwrap();

    index::Folder::new(folder_hash, files)
}

pub fn read_directories(file: &mut File, index_info: &IndexInfo) -> HashMap<u32, index::Folder> {
    let current_pos = file.seek(SeekFrom::Current(0)).unwrap();

    let mut folders = HashMap::<u32, index::Folder>::new();
    for i in 0..index_info.folders_count {
        let folder = read_folder(file, index_info.folders_offset + i * 0x10);
        folders.insert(folder.folder_hash, folder);
    }

    file.seek(SeekFrom::Start(current_pos)).unwrap();

    folders
}