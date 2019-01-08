mod info;
mod row_reader;
pub use self::row_reader::*;
pub mod decoding;
pub mod ex;

use std::error::Error;

use self::ex::SheetDataType;

use std::rc::Rc;
use std::io::Write;

use indexmap::IndexMap;


pub struct Sheet {
    pub rows: IndexMap<usize, SheetRow>,
    pub types: Rc<Vec<SheetDataType>>,
    pub column_count: u32
}

pub struct SheetRow {
    pub by: Vec<u8>,
    pub types: Rc<Vec<SheetDataType>>
}




pub trait FromSheet: Sized + std::fmt::Debug {
    type Error;
    fn from_ex_data(b: &SheetRow, cell: usize) -> Result<Self, Self::Error>;
}

#[derive(Debug)]
pub enum SheetErrorType {
    Incompatible,
    CellOutOfBounds,
    StringProcessing
}

#[derive(Debug)]
pub struct SheetError {
    pub error_type: SheetErrorType
}

impl Error for SheetError {}

impl std::fmt::Display for SheetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.error_type {
            SheetErrorType::Incompatible => write!(f, "The type was invalid."),
            SheetErrorType::StringProcessing => write!(f, "There was a problem converting the string to UTF-8."),
            SheetErrorType::CellOutOfBounds => write!(f, "The specified cell was out of bounds.")
        }


    }
}

impl SheetRow {
    pub fn read_cell_data<T: FromSheet + std::fmt::Debug>(&self, cell: usize) -> Result<T, T::Error> {
        T::from_ex_data(self, cell)
    }
}

pub fn write_csv(sheet: &Sheet, buffer: &mut Write) -> Result<(), ::FFXIVError> {
    write!(buffer, "\"index\",")?;
    for (index, typ) in sheet.types.iter().enumerate() {
        if index == sheet.types.len() - 1 {
            write!(buffer, "\"{}\"", typ.get_header())
        } else {
            write!(buffer, "\"{}\",", typ.get_header())
        }?;
    }
    writeln!(buffer, "")?;;
    for (index, row) in sheet.rows.iter() {
        write!(buffer, "\"{}\",", index)?;
        for (index_typ, typ) in row.types.iter().enumerate() {
            use ::sheet::ex::SheetDataType;
            use ::sheet::BitFlags;
            match typ {
                SheetDataType::String(_s_info) =>
                    write!(buffer, "\"{}\"", row.read_cell_data::<String>(index_typ)?),
                SheetDataType::Bool(_info) =>
                    write!(buffer, "\"{}\"", row.read_cell_data::<bool>(index_typ)?),
                SheetDataType::Byte(_info) =>
                    write!(buffer, "\"{}\"", row.read_cell_data::<i8>(index_typ)?),
                SheetDataType::UByte(_info) =>
                    write!(buffer, "\"{}\"", row.read_cell_data::<u8>(index_typ)?),
                SheetDataType::Short(_info) =>
                    write!(buffer, "\"{}\"", row.read_cell_data::<i16>(index_typ)?),
                SheetDataType::UShort(_info) =>
                    write!(buffer, "\"{}\"", row.read_cell_data::<u16>(index_typ)?),
                SheetDataType::Int(_info) =>
                    write!(buffer, "\"{}\"", row.read_cell_data::<i32>(index_typ)?),
                SheetDataType::UInt(_info) =>
                    write!(buffer, "\"{}\"", row.read_cell_data::<u32>(index_typ)?),
                SheetDataType::Float(_info) =>
                    write!(buffer, "\"{}\"", row.read_cell_data::<f32>(index_typ)?),
                SheetDataType::PackedInts(_info) =>
                    write!(buffer, "\"unsupported\""),
                SheetDataType::BitFlags(b_info) =>
                    write!(buffer, "\"{}\"", row.read_cell_data
                        ::<BitFlags>(index_typ)?.get_bool(b_info.bit.clone())),

            }?;
            if index_typ != row.types.len() - 1 {
                write!(buffer, ",")?;
            }
        }
        writeln!(buffer, "")?;
    }
    Ok(())
}