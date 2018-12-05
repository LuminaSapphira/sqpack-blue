mod info;
mod decoding;
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

/*

{
let sheet = decode_sheet(vec<u8>) -> Sheet;
sheet.whateveR()

}



*/



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

impl FromSheet for u32 {
    type Error = SheetError;
    fn from_ex_data(b: &SheetRow, cell: usize) -> Result<Self, Self::Error> {
        match b.types.get(cell) {
            Some(get_result) => match get_result {
                SheetDataType::UInt(info) => {
                    let end: usize = info.pointer as usize + 4;
                    Ok(b.by[info.pointer as usize .. end].iter().enumerate().fold(0, |acc, x| acc + (((*x.1) as u32) << ((x.0) as usize) * 8)))
                },
                _ => Err(Self::Error { error_type: SheetErrorType::Incompatible })
            },
            None => Err(Self::Error{error_type: SheetErrorType::CellOutOfBounds})
        }
    }
}

impl FromSheet for i32 {
    type Error = SheetError;
    fn from_ex_data(b: &SheetRow, cell: usize) -> Result<Self, Self::Error> {
        match b.types.get(cell) {
            Some(get_result) => match get_result {
                SheetDataType::Int(info) => {
                    let end: usize = info.pointer as usize + 4;
                    Ok(b.by[info.pointer as usize..end].iter().enumerate().fold(0, |acc, x| acc + (((*x.1) as u32) << ((x.0) as usize) * 8)) as i32)
                },
                _ => Err(Self::Error { error_type: SheetErrorType::Incompatible })
            },
            None => Err(Self::Error{error_type: SheetErrorType::CellOutOfBounds})
        }
    }
}

impl FromSheet for String {
    type Error = SheetError;
    fn from_ex_data(b: &SheetRow, cell: usize) -> Result<Self, Self::Error> {
        match b.types.get(cell) {
            Some(get_result) => match get_result {
                SheetDataType::String(info) => {
                    let end: usize = info.pointer as usize + 4;
                    let sptr = b.by[info.pointer as usize..end].iter().enumerate().fold(0, |acc, x| acc + (((*x.1) as u32) << ((x.0) as usize) * 8));
                    let length_option = b.by[info.strings_offset as usize + sptr as usize..].iter().enumerate().filter(|x| x.1 == &0).map(|x| x.0).nth(0);
                    let strend = match length_option {
                        Some(val) => sptr as usize + val as usize + info.strings_offset as usize,
                        None => b.by.len()
                    };
                    match Self::from_utf8((b.by[(info.strings_offset + sptr) as usize..strend]).to_vec().clone()) {
                        Ok(val) => Ok(val),
                        _ => Err(Self::Error { error_type: SheetErrorType::StringProcessing })
                    }
                },
                _ => Err(Self::Error { error_type: SheetErrorType::Incompatible })
            },
            None => Err(Self::Error{error_type: SheetErrorType::CellOutOfBounds})
        }
    }
}

impl SheetRow {
    pub fn ex_data_into<T: FromSheet + std::fmt::Debug>(&self, cell: usize) -> Result<T, T::Error> {
        T::from_ex_data(self, cell)
    }
}