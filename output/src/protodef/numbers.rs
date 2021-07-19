#![allow(dead_code)]
use super::Protodef;

use crate::comparer;

use crate::count_all;
use crate::count_float;
use crate::count_signed;

//uints, ints, floats, varint.

macro_rules! counter {
    {$id: ident, $kind: ident} => {
        pub fn parse(input: &mut &[u8]) -> Option<Protodef> {
            Some(Protodef::$id(real_parse(input)?))
        }
        pub fn serial(data: &Protodef, output: &mut Vec<u8>) -> Option<()> {
            if let Protodef::$id(val) = data {
                real_serial(val, output);
                Some(())
            } else {
                None
            }
        }
        pub fn usize_parse(input: &mut &[u8]) -> Option<usize> {
            let value = real_parse(input)?;
            comparer!($kind, &value)
        }
        pub fn usize_serial(num: usize, output: &mut Vec<u8>) -> Option<()> {
            //Check if the serialized length prefix type
            //can hold a value as big as the current usize.
            if num as u64 <= $kind::MAX as u64 {
                let data = num as $kind;
                real_serial(&data, output);
                Some(())
            } else {
                None
            }
        }
    }
}

macro_rules! full_num {
    {$id: ident, $kind: ident, $read: ident} => {
        pub mod $kind {
            use super::*;
            counter! {$id, $kind}
            pub fn real_parse(input: &mut &[u8]) -> Option<$kind> {
                let bytes = array_bits::$read(input)?;
                Some($kind::from_be_bytes(bytes))
            }
            pub fn real_serial(data: &$kind, output: &mut Vec<u8>) {
                output.extend(&data.to_be_bytes());
            }
        }
    };
}

pub mod types {
    use super::*;
    //Unsigned integers:
    full_num! {Uint8, u8, read_8}
    full_num! {Uint16, u16, read_16}
    full_num! {Uint32, u32, read_32}
    full_num! {Uint64, u64, read_64}
    //Signed integers:
    full_num! {Int8, i8, read_8}
    full_num! {Int16, i16, read_16}
    full_num! {Int32, i32, read_32}
    full_num! {Int64, i64, read_64}
    //Floating point numbers:
    full_num! {Float, f32, read_32}
    full_num! {Double, f64, read_64}
    pub mod varint {
        use super::*;
        counter! {Int32, i32}
        pub fn real_parse(input: &mut &[u8]) -> Option<i32> {
            let mut res = 0_u32;
            let mut shift = 0_u32;
            for i in 0..=4 {
                let byte = *input.get(i)?;
                let val = byte & 0x7F;
                res |= (val as u32) << shift;

                if val == byte {
                    if i == 4 && (val & 0xF0 != 0) {
                        return None;
                    }
                    *input = &input[i + 1..];
                    return Some(res as i32);
                }
                shift += 7;
            }
            return None;
        }
        pub fn real_serial(data: &i32, output: &mut Vec<u8>) {
            let mut val = *data as u32;
            while (val >> 7) != 0 {
                output.push(((val & 0x7F) | 0x80) as u8);
                val = val >> 7;
            }
            output.push(val as u8);
        }
    }
}

mod array_bits {
    pub fn read_8(input: &mut &[u8]) -> Option<[u8; 1]> {
        let byte = input.get(0)?;
        *input = &input[1..];
        Some([*byte])
    }
    pub fn read_16(input: &mut &[u8]) -> Option<[u8; 2]> {
        let bytes = input.get(..2)?;
        *input = &input[2..];
        Some([bytes[0], bytes[1]])
    }
    pub fn read_32(input: &mut &[u8]) -> Option<[u8; 4]> {
        let bytes = input.get(..4)?;
        let mut array = [0; 4];
        array.copy_from_slice(bytes);
        *input = &input[4..];
        Some(array)
    }
    pub fn read_64(input: &mut &[u8]) -> Option<[u8; 8]> {
        let bytes = input.get(..8)?;
        let mut array = [0; 8];
        array.copy_from_slice(bytes);
        *input = &input[8..];
        Some(array)
    }
}
