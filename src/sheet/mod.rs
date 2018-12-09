mod info;
mod row_reader;
pub use self::row_reader::*;
pub mod decoding;
pub mod ex;

use std::error::Error;

use self::ex::SheetDataType;

use std::rc::Rc;


pub struct Sheet {
    pub rows: Vec<SheetRow>,
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