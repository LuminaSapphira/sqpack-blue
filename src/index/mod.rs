use std::collections::HashMap;

pub struct Index {
    folders: HashMap<u32, Folder>
}

pub struct SheetIndex {
    index: Index
}

pub struct Folder {
    pub folder_hash: u32,
    sub_files: HashMap<u32, File>
}

pub struct File {
    pub folder_hash: u32,
    pub file_hash: u32,
    pub data_offset: u32,
    pub dat_file: u8
}

impl Index {

    pub fn new(folders: HashMap<u32, Folder>) -> Index {
        Index { folders }
    }

    pub fn get_folder(&self, folder_hash: u32) -> Option<&Folder> {
        self.folders.get(&folder_hash)
    }

    pub fn get_file(&self, folder_hash: u32, file_hash: u32) -> Option<&File> {
        let m = self.get_folder(folder_hash);
        match m {
            None => None,
            Some(folder) => folder.get_file(file_hash)
        }

    }

}

impl SheetIndex {
    pub fn new(index: Index) -> SheetIndex {
        SheetIndex{index}
    }
}

impl Folder {
    pub fn new(hash: u32, files: HashMap<u32, File>) -> Folder {
        Folder {folder_hash: hash, sub_files: files}
    }

    pub fn get_file(&self, file_hash: u32) -> Option<&File> {
        self.sub_files.get(&file_hash)
    }

}