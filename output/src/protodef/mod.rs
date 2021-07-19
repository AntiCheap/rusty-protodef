#![allow(dead_code)]
#![allow(unused_variables)]

mod macros;

use crate::count_all;
use crate::count_float;
use crate::count_signed;

mod numbers;
mod primitives;

pub mod types {
    use super::*;
    pub use numbers::types::*;
    pub use primitives::types::*;
}

use std::collections::HashMap;
type Object = Box<HashMap<String, Protodef>>;

#[derive(Debug)]
pub enum Protodef {
    Object(Object),
    Array(Vec<Protodef>),
    Bool(bool),
    Buffer(Vec<u8>),
    String(String),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float(f32),
    Double(f64),
    Void(),
}

//Navigation
impl Protodef {
    pub fn new_object() -> Self {
        Self::Object(Box::new(HashMap::new()))
    }
    pub fn get(&self, field: &str) -> Option<&Self> {
        match self {
            Self::Object(x) => x.get(field),
            _ => None,
        }
    }
    pub fn get_mut(&mut self, field: &str) -> Option<&mut Self> {
        match self {
            Self::Object(x) => x.get_mut(field),
            _ => None,
        }
    }
    pub fn set(&mut self, field: &str, value: Self) {
        match self {
            Self::Object(x) => {
                x.insert(field.to_string(), value);
            }
            _ => panic!(),
        }
    }
}

//Referencing
impl Protodef {
    // pub fn as_array(&self) -> Option<&Vec<Protodef>> {
    //     match self {
    //         Self::Array(val) => Some(val),
    //         _ => None,
    //     }
    // }
    // pub fn as_buffer(&self) -> Option<&Vec<u8>> {
    //     match self {
    //         Self::Buffer(val) => Some(val),
    //         _ => None,
    //     }
    // }
    // pub fn as_string(&self) -> Option<&String> {
    //     match self {
    //         Self::String(val) => Some(val),
    //         _ => None,
    //     }
    // }
    // pub fn as_bool(&self) -> Option<&bool> {
    //     match self {
    //         Self::Bool(val) => Some(val),
    //         _ => None,
    //     }
    // }
    // pub fn as_u8(&self) -> Option<&u8> {
    //     match self {
    //         Self::Uint8(val) => Some(val),
    //         _ => None,
    //     }
    // }
    // pub fn as_u16(&self) -> Option<&u16> {
    //     match self {
    //         Self::Uint16(val) => Some(val),
    //         _ => None,
    //     }
    // }
    // pub fn as_u32(&self) -> Option<&u32> {
    //     match self {
    //         Self::Uint32(val) => Some(val),
    //         _ => None,
    //     }
    // }
    // pub fn as_u64(&self) -> Option<&u64> {
    //     match self {
    //         Self::Uint64(val) => Some(val),
    //         _ => None,
    //     }
    // }
    // pub fn as_i8(&self) -> Option<&i8> {
    //     match self {
    //         Self::Int8(val) => Some(val),
    //         _ => None,
    //     }
    // }
    // pub fn as_i16(&self) -> Option<&i16> {
    //     match self {
    //         Self::Int16(val) => Some(val),
    //         _ => None,
    //     }
    // }
    // pub fn as_i32(&self) -> Option<&i32> {
    //     match self {
    //         Self::Int32(val) => Some(val),
    //         _ => None,
    //     }
    // }
    // pub fn as_i64(&self) -> Option<&i64> {
    //     match self {
    //         Self::Int64(val) => Some(val),
    //         _ => None,
    //     }
    // }
    // pub fn as_f32(&self) -> Option<&f32> {
    //     match self {
    //         Self::Float(val) => Some(val),
    //         _ => None,
    //     }
    // }
    // pub fn as_f64(&self) -> Option<&f64> {
    //     match self {
    //         Self::Double(val) => Some(val),
    //         _ => None,
    //     }
    // }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(val) => Some(val),
            _ => None,
        }
    }
    pub fn to_length(&self) -> Option<usize> {
        match self {
            //string.len() nicely returns bytes length.
            Protodef::String(val) => Some(val.len()),
            Protodef::Buffer(val) => Some(val.len()),
            Protodef::Array(val) => Some(val.len()),
            _ => None,
        }
    }
    pub fn to_count(&self) -> Option<usize> {
        match self {
            //Implemented by all types of numbers.
            Protodef::Uint8(val) => count_all!(val),
            Protodef::Uint16(val) => count_all!(val),
            Protodef::Uint32(val) => count_all!(val),
            Protodef::Uint64(val) => count_all!(val),
            Protodef::Int8(val) => count_signed!(val),
            Protodef::Int16(val) => count_signed!(val),
            Protodef::Int32(val) => count_signed!(val),
            Protodef::Int64(val) => count_signed!(val),
            Protodef::Float(val) => count_float!(val),
            Protodef::Double(val) => count_float!(val),
            _ => None,
        }
    }
    pub fn to_compare<'a, T>(&'a self, func: fn(x: &str) -> Option<T>) -> Option<T> {
        match self {
            //Numbers plus bool and string (pstring or cstring).
            Protodef::Uint8(val) => func(&val.to_string()[..]),
            Protodef::Uint16(val) => func(&val.to_string()[..]),
            Protodef::Uint32(val) => func(&val.to_string()[..]),
            Protodef::Uint64(val) => func(&val.to_string()[..]),
            Protodef::Int8(val) => func(&val.to_string()[..]),
            Protodef::Int16(val) => func(&val.to_string()[..]),
            Protodef::Int32(val) => func(&val.to_string()[..]),
            Protodef::Int64(val) => func(&val.to_string()[..]),
            Protodef::Float(val) => func(&val.to_string()[..]),
            Protodef::Double(val) => func(&val.to_string()[..]),
            Protodef::Bool(val) => func(&val.to_string()[..]),
            Protodef::String(val) => func(&val),
            _ => None,
        }
    }
}
