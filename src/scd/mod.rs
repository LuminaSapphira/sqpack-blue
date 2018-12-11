
mod entry;
mod entry_ogg;
mod entry_msadpcm;
mod decoding;

use self::entry::*;
//use self::entry_ogg::SCDEntryOgg;
//use self::entry_msadpcm::SCDEntryMSADPCM;
use self::decoding::*;

use ::FFXIVError;

pub struct SCDFile {
    pub entries: Vec<Box<SCDEntry>>,
    pub header: SCDHeader,
}

pub struct SCDHeader {
    pub unknown_1_count: i16,
    pub unknown_2_count: i16,
    pub entry_count: i16,
    pub unknown_1: i16,
    pub unknown_1_offset: i32,
    pub entry_table_offset: i32,
    pub unknown_2_offset: i32,
    pub unknown_2: i32,
    pub unknown_offset_1: i32,
}

impl SCDFile {
    pub fn decode(data: Vec<u8>) -> Result<SCDFile, FFXIVError> {
        let little_endian = decode_little_endianness(&data)?;
        let header = decode_scd_header(&data, &little_endian)?;
        let entries = decode_scd_entries(&data, &header, &little_endian)?;


        Ok(SCDFile{entries, header})
    }
}

#[cfg(test)]
mod scd_tests {
    extern crate md5;
    use super::*;

    #[test]
    fn scd_ogg_export() {
//        4016BA963FDC4EE8D73A69656029F1F7
        use std::io::Read;
        //use std::io::Write;
        use std::fs::File;
        let path: String = std::env::var("test_files_path").unwrap();
        let mut scd = path.clone();
        scd.push_str("bgm_con_bahamut_bigboss1.scd");
        let mut file = File::open(&scd).unwrap();
        let mut data = Vec::<u8>::new();
        file.read_to_end(&mut data).unwrap();

        let decoded = SCDFile::decode(data).unwrap();

        let scd_entry = decoded.entries[0].decoded();

        let expected: [u8;16] = [0x40, 0x16, 0xBA, 0x96, 0x3F, 0xDC, 0x4E, 0xE8, 0xD7, 0x3A, 0x69, 0x65, 0x60, 0x29, 0xF1, 0xF7];
        let digest:[u8;16] = md5::compute(scd_entry).into();
        assert_eq!(expected, digest);

    }

    #[test]
    fn scd_wav_export() {
        // 7E02DF98615F6296DC618515AB6C061E
        use std::io::Read;
//        use std::io::Write;
        use std::fs::File;
        let path: String = std::env::var("test_files_path").unwrap();
        let mut scd = path.clone();
        scd.push_str("zingle_wedding_complete.scd");
        let mut file = File::open(&scd).unwrap();
        let mut data = Vec::<u8>::new();
        file.read_to_end(&mut data).unwrap();

        let mut ogg = path.clone();
        ogg.push_str("zingle_wedding_complete.wav");
//        let mut ogg_file = File::create(&ogg).unwrap();

        let decoded = SCDFile::decode(data).unwrap();

        let scd_entry = decoded.entries[0].decoded();
        let expected: [u8;16] = [0x7E, 0x02, 0xDF, 0x98, 0x61, 0x5F, 0x62, 0x96, 0xDC, 0x61, 0x85, 0x15, 0xAB, 0x6C, 0x06, 0x1E];
        let digest:[u8;16] = md5::compute(scd_entry).into();
        assert_eq!(expected, digest);

    }

}