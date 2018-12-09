use ::FFXIVError;

#[derive(Debug)]
pub struct SCDEntryHeader {
    pub data_size: i32,
    pub channel_count: i32,
    pub frequency: i32,
    pub codec: SCDCodec,
    pub loop_start: i32,
    pub loop_end: i32,
    pub samples_offset: i32,
    pub aux_chunk_count: i16,
    pub unknown_1: i16,
}

#[derive(Debug)]
pub enum SCDCodec {
    None,
    OGG,
    MSADPCM,
}

impl std::fmt::Display for SCDCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SCDCodec::None => write!(f, "[SCDCodec::None]"),
            SCDCodec::OGG => write!(f, "[SCDCodec::OGG]"),
            SCDCodec::MSADPCM => write!(f, "[SCDCodec::MSADPCM]"),
        }
    }
}


pub trait SCDEntry {
    fn create(buffer: &[u8], header: SCDEntryHeader, chunks_offset: &u32, data_offset: &u32, little_end: &bool) -> Result<Box<Self>, FFXIVError> where Self: Sized;
    fn decoded(&self) -> &Vec<u8>;
    fn header(&self) -> &SCDEntryHeader;
}

pub struct SCDEntryNone {
    header: SCDEntryHeader,
    decoded: Vec<u8>
}

impl SCDEntry for SCDEntryNone {
    fn create(buffer: &[u8], header: SCDEntryHeader, chunks_offset: &u32, data_offset: &u32, little_end: &bool) -> Result<Box<Self>, FFXIVError> {
        Ok(Box::new(SCDEntryNone{

            header: SCDEntryHeader {
                data_size: 0, channel_count: 0,
                frequency: 0, codec: SCDCodec::None,
                loop_start: 0, loop_end: 0,
                samples_offset: 0, aux_chunk_count: 0,
                unknown_1: 0,
            },
            decoded: vec![]
        }))
    }

    fn decoded(&self) -> &Vec<u8> {
        &self.decoded
    }

    fn header(&self) -> &SCDEntryHeader {
        &self.header
    }
}