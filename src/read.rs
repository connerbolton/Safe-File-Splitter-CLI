/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{fs::File, io::{BufRead, BufReader}};

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn read_line_safely(buf_reader: &mut BufReader<File>) -> String {

    let mut line = String::new();

    loop {

        let bytes_read = buf_reader.read_line(&mut line).unwrap();
        if bytes_read == 0 { break; }
        
        let mut inside_quotes = false;
        let bytes = line.as_bytes();
        let mut index = 0;

        while index < bytes.len() {

            if bytes[index] == b'"' {
                if index + 1 < bytes.len() && bytes[index + 1] == b'"' {
                    index += 1;
                } else {
                    inside_quotes = !inside_quotes;
                }
            }

            index += 1;

        }

        if !inside_quotes { break; }

    }

    return line;

}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////