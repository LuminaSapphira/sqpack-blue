use super::{FromSheet, SheetError, SheetDataType, SheetRow, SheetErrorType};
use ::byteorder::ByteOrder;
use ::byteorder::BigEndian;

#[derive(Debug)]
pub struct BitFlags {
    pub data: u8,
}

impl BitFlags {
    pub fn get_bool(&self, bit: u8) -> bool {
        assert!(bit < 8);
        ((self.data >> bit) & 0x1 as u8) == 0x1
    }
}

impl std::fmt::Display for BitFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..8 as u8 {
            write!(f, "{}", if self.get_bool(i) { 1 } else { 0 });
        }
        write!(f, "")
    }
}

impl FromSheet for u32 {
    type Error = SheetError;
    fn from_ex_data(b: &SheetRow, cell: usize) -> Result<Self, Self::Error> {
        match b.types.get(cell) {
            Some(get_result) => match get_result {
                SheetDataType::UInt(info) => {
                    let end: usize = info.pointer as usize + 4;
                    Ok(BigEndian::read_u32(&b.by[info.pointer as usize .. end]))
//                    Ok(b.by[info.pointer as usize .. end].iter().enumerate().fold(0, |acc, x| acc + (((*x.1) as u32) << ((x.0) as usize) * 8)))
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
                    Ok(BigEndian::read_i32(&b.by[info.pointer as usize .. end]))
//                    Ok(b.by[info.pointer as usize..end].iter().enumerate().fold(0, |acc, x| acc + (((*x.1) as u32) << ((x.0) as usize) * 8)) as i32)
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

impl FromSheet for u8 {
    type Error = SheetError;
    fn from_ex_data(b: &SheetRow, cell: usize) -> Result<Self, Self::Error> {
        match b.types.get(cell) {
            Some(get_result) => match get_result {
                SheetDataType::UByte(info) => {
                    Ok(b.by[info.pointer as usize].clone())
                },
                _ => Err(Self::Error { error_type: SheetErrorType::Incompatible })
            },
            None => Err(Self::Error{error_type: SheetErrorType::CellOutOfBounds})
        }
    }
}

impl FromSheet for i8 {
    type Error = SheetError;
    fn from_ex_data(b: &SheetRow, cell: usize) -> Result<Self, Self::Error> {
        match b.types.get(cell) {
            Some(get_result) => match get_result {
                SheetDataType::Byte(info) => {
                    Ok(b.by[info.pointer as usize].clone() as i8)
                },
                _ => Err(Self::Error { error_type: SheetErrorType::Incompatible })
            },
            None => Err(Self::Error{error_type: SheetErrorType::CellOutOfBounds})
        }
    }
}

impl FromSheet for u16 {
    type Error = SheetError;
    fn from_ex_data(b: &SheetRow, cell: usize) -> Result<Self, Self::Error> {
        match b.types.get(cell) {
            Some(get_result) => match get_result {
                SheetDataType::UShort(info) => {
                    let end: usize = info.pointer as usize + 2;
                    Ok(BigEndian::read_u16(&b.by[info.pointer as usize .. end]))
                },
                _ => Err(Self::Error { error_type: SheetErrorType::Incompatible })
            },
            None => Err(Self::Error{error_type: SheetErrorType::CellOutOfBounds})
        }
    }
}

impl FromSheet for i16 {
    type Error = SheetError;
    fn from_ex_data(b: &SheetRow, cell: usize) -> Result<Self, Self::Error> {
        match b.types.get(cell) {
            Some(get_result) => match get_result {
                SheetDataType::Short(info) => {
                    let end: usize = info.pointer as usize + 2;
                    Ok(BigEndian::read_i16(&b.by[info.pointer as usize .. end]))
                },
                _ => Err(Self::Error { error_type: SheetErrorType::Incompatible })
            },
            None => Err(Self::Error{error_type: SheetErrorType::CellOutOfBounds})
        }
    }
}

impl FromSheet for bool {
    type Error = SheetError;
    fn from_ex_data(b: &SheetRow, cell: usize) -> Result<Self, Self::Error> {
        match b.types.get(cell) {
            Some(get_result) => match get_result {
                SheetDataType::Bool(info) => {
                    let b = &b.by[info.pointer as usize];
                    if *b == 0x00 {
                        Ok(false)
                    }
                    else {
                        Ok(true)
                    }
                },
                _ => Err(Self::Error { error_type: SheetErrorType::Incompatible })
            },
            None => Err(Self::Error{error_type: SheetErrorType::CellOutOfBounds})
        }
    }
}

impl FromSheet for f32 {
    type Error = SheetError;
    fn from_ex_data(b: &SheetRow, cell: usize) -> Result<Self, Self::Error> {
        match b.types.get(cell) {
            Some(get_result) => match get_result {
                SheetDataType::Float(info) => {
                    let end: usize = info.pointer as usize + 4;
                    Ok(BigEndian::read_f32(&b.by[info.pointer as usize .. end]))
                },
                _ => Err(Self::Error { error_type: SheetErrorType::Incompatible })
            },
            None => Err(Self::Error{error_type: SheetErrorType::CellOutOfBounds})
        }
    }
}


impl FromSheet for BitFlags {
    type Error = SheetError;
    fn from_ex_data(b: &SheetRow, cell: usize) -> Result<Self, Self::Error> {
        match b.types.get(cell) {
            Some(get_result) => match get_result {
                SheetDataType::BitFlags(b_info) => {
                    let data = b.by[b_info.pointer as usize].clone();
                    let bf = BitFlags{ data };
                    Ok(bf)
                },
                _ => Err(Self::Error { error_type: SheetErrorType::Incompatible })
            },
            None => Err(Self::Error { error_type: SheetErrorType::CellOutOfBounds })
        }
    }
}