use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for byte in io::stdin().bytes() {
        match byte {
            Ok(byte) => {
                let character = byte as char;
                if character.is_control() {
                    println!("{:?} \r", byte);
                } else {
                    println!("{:?} ({})\r", byte, character);
                }
                if byte == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(err) => panic!("{}", err)
        }
    }
}