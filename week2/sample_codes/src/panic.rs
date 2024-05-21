use std::fs::{File, write};
use std::io::{self, BufRead, BufReader};

fn write_to_file() -> io::Result<()> {
    write("foo.text", b"Testing write to file")?; // ? will return the error if it exists
    Ok(())
}

fn handle_write_error(error : io::Error) {
    match error.kind() {
        io::ErrorKind::PermissionDenied => {
            panic!("Can't write a file error: {}", error);
        }
        _ => {
            panic!("Writing Error: {}", error);
        }
    }
}

fn handle_file_error(error: io::Error) -> File {
    match error.kind() {
        io::ErrorKind::NotFound => {
            panic!("File not found: {}", error);
        }
        io::ErrorKind::PermissionDenied => {
            panic!("No Permission to open the file: {}", error);
        }
        _ => {
            panic!("Error Openning a File: {}", error);
        }
    }
}

fn main() {
    if let Err(error) = write_to_file() {
        handle_write_error(error);
    }

    let file = match File::open("foo.txt") {
        Ok(file) => file,
        Err(error) => handle_file_error(error),
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading a line: {}", error);
            }
        };
    }
}

