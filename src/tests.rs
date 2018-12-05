
extern crate md5;
#[cfg(test)]
mod basic {
    use super::super::*;
//    use std::time::{Duration, SystemTime};

    #[test]
    fn test_initialize_ffxiv() {
        let path = std::env::var("sqpack").unwrap();

        let g =
            FFXIV::new(Path::new(&path));
        g.unwrap();
    }

    #[test]
    fn test_decoding() {
        let mut path = std::env::var("sqpack").unwrap();
        path.push_str("/ffxiv/0a0000.win32.index");

        let mut file = File::open(&path).expect("not found");
        let i = io::read_index_file(&mut file).unwrap();
        let exd = i.get_file(0xE39B7999, 0xa41d4329)
            .expect("couldn't unwrap file in lib.rs");
        assert_eq!(exd.data_offset, 104770944);
    }

    #[test]
    fn test_manual_export() {

        let path = std::env::var("sqpack").unwrap();
        let mut path_index = path.clone();
        let mut path_data = path.clone();
        path_index.push_str("/ffxiv/0c0000.win32.index");
        path_data.push_str("/ffxiv/0c0000.win32.dat0");

        let mut index = File::open(&path_index).expect("not found");
        let index_scd = io::read_index_file(&mut index).unwrap();
        let scd_file_index = index_scd.get_file(0x0AF269D6, 0xe3b71579).unwrap();

        let mut dat_file =
            File::open(&path_data).expect("not found");
        let scd = io::read_data_file(&mut dat_file, scd_file_index).unwrap();

//        use std::io::Write;
        let expected: [u8;16] = [0x43, 0x51, 0x52, 0x41, 0xA8, 0xE7, 0x8E, 0xCC, 0xD5, 0xE1, 0xB3, 0x3A, 0xBE, 0x89, 0xDB, 0xCC];
        let digest:[u8;16] = md5::compute(&scd).into();
        assert_eq!(expected, digest);
    }

    #[test]
    fn test_index_location() {
        let path = std::env::var("sqpack").unwrap();
        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();
        let exfile = ffxiv.get_exfile(&String::from("music/ffxiv/bgm_system_title.scd")).unwrap();
        assert_eq!(exfile.get_index_file(ffxiv.path.as_path()).as_os_str(),
            "C:\\Program Files (x86)\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn\\game\\sqpack\\ffxiv\\0c0000.win32.index"
        );
    }

    #[test]
    fn test_get_index() {
        let path = std::env::var("sqpack").unwrap();
        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();
        ffxiv.get_index(&ffxiv.get_exfile(&String::from("music/ffxiv/bgm_system_title.scd")).unwrap()).unwrap();
    }

    #[test]
    fn test_dat_file_identification() {
        let path = std::env::var("sqpack").unwrap();
        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();
        let exfile = ffxiv.get_exfile(&String::from("music/ffxiv/bgm_system_title.scd")).unwrap();
        let index_file =
            ffxiv.get_index(&exfile).unwrap();
        let phash = exfile.get_sqpack_hashcode();
        let ifl = index_file.get_file(phash.folder_hash, phash.file_hash).unwrap();
        let base_dat_path= exfile.get_dat_file(ffxiv.path.as_path(), ifl.dat_file);
        assert_eq!(
            "C:\\Program Files (x86)\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn\\game\\sqpack\\ffxiv\\0c0000.win32.dat0",
            base_dat_path.as_os_str()
        );
    }

    #[test]
    fn test_export_raw_data() {
        let path = std::env::var("sqpack").unwrap();

        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();
        let v = ffxiv.get_raw_data(
            &ExFileIdentifier::new(
                &String::from("music/ffxiv/bgm_system_title.scd")).unwrap()).unwrap();

        let expected: [u8;16] = [0x43, 0x51, 0x52, 0x41, 0xA8, 0xE7, 0x8E, 0xCC, 0xD5, 0xE1, 0xB3, 0x3A, 0xBE, 0x89, 0xDB, 0xCC];
        let digest:[u8;16] = md5::compute(&v.0).into();
        assert_eq!(expected, digest);
    }

    #[test]
    fn sheet_index() {
        let path = std::env::var("sqpack").unwrap();

        let ffxiv = FFXIV::new(Path::new(&path)).unwrap();

        let s = ffxiv.get_sheet_index().unwrap();
    }

}

#[cfg(test)]
mod hash {
    use super::super::*;

    #[test]
    fn test_hash_file_name() {
        assert_eq!(hash::compute(&String::from("bgm_system_title.scd")), 0xE3B71579)
    }

    #[test]
    fn test_hash_folder_name() {
        assert_eq!(hash::compute(&String::from("music/ffxiv")), 0x0AF269D6)
    }

    #[test]
    fn test_hash_path() {
        let hash::PathHash{folder_hash, file_hash} = hash::compute_path(&String::from("music/ffxiv/bgm_system_title.scd"));
        assert_eq!(folder_hash, 0x0AF269D6);
        assert_eq!(file_hash, 0xE3B71579)
    }

    #[test]
    fn test_hash_lower_eq() {
        assert_eq!(hash::compute(&String::from("bgm_system_title.scd")), hash::compute(&String::from("BGM_System_Title.scd")));
    }

}

#[cfg(test)]
mod expack_test {
    use super::super::*;

    #[test]
    fn test_expack() {
        let path = std::env::var("sqpack").unwrap();
        let apath = Path::new(&path);

        let m = expack::ExFileIdentifier::new(&String::from("music/ex2/BGM_EX2_Dan_D09.scd")).unwrap();
        let pbuff = m.get_index_file(apath);

        let a = pbuff.as_os_str();
        println!("{:?}", a);
        assert_eq!(a, "C:\\Program Files (x86)\\SquareEnix\\FINAL FANTASY XIV - A Realm Reborn\\game\\sqpack\\ex2\\0c0200.win32.index");

    }
}

