#![allow(dead_code)]
use super::Protodef;

//bool, cstring, (void).

pub mod types {
    use super::*;
    pub mod bool {
        use super::*;
        pub fn real_parse(input: &mut &[u8]) -> Option<bool> {
            let byte = *input.get(0)?;
            *input = &input[1..];
            match byte {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            }
        }
        pub fn real_serial(data: &bool, output: &mut Vec<u8>) {
            output.push(*data as u8);
        }
        pub fn parse(input: &mut &[u8]) -> Option<Protodef> {
            Some(Protodef::Bool(real_parse(input)?))
        }
        pub fn serial(data: &Protodef, output: &mut Vec<u8>) -> Option<()> {
            if let Protodef::Bool(val) = data {
                real_serial(val, output);
                Some(())
            } else {
                None
            }
        }
    }
    pub mod cstring {
        use super::*;
        pub fn real_parse(input: &mut &[u8]) -> Option<String> {
            let till = input.iter().position(|&x| x == 0)?;
            let data = input[..till].to_vec();
            *input = &input[till + 1..];
            String::from_utf8(data).ok()
        }
        pub fn real_serial(data: &String, output: &mut Vec<u8>) {
            output.extend_from_slice(data.as_bytes());
            output.push(0);
        }
        pub fn parse(input: &mut &[u8]) -> Option<Protodef> {
            Some(Protodef::String(real_parse(input)?))
        }
        pub fn serial(data: &Protodef, output: &mut Vec<u8>) -> Option<()> {
            if let Protodef::String(val) = data {
                real_serial(val, output);
                Some(())
            } else {
                None
            }
        }
    }
}
