use std::fs::{File, write};
use std::io::{self, BufRead, BufReader};
use clap::{Arg, Command};


fn write_to_file(path: & str) -> io::Result<()> {
    write(path, b"Hello, World!\n")?;
    Ok(())
}

fn handle_write_error(error: io::Error) {
    match error.kind() {
        io::ErrorKind::PermissionDenied => {
            panic!("Can't write  file at destination: {}", error);
        }
        _ => {
            panic!("An error occurred while writing to file: {}", error);
        }
    }
}

fn read_file(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn handle_read_error(error: io::Error) {
    match error.kind() {
        io::ErrorKind::NotFound => {
            panic!("File not found: {}", error);
        },
        io::ErrorKind::PermissionDenied => {
            panic!("Can't read file at destination: {}", error);
        },
        _ => {
            panic!("An error occurred while reading the file: {}", error);
        }
    }
}

fn main() {
    let matches = Command::new("File Handler")
    .version("1.0")
    .author("Tamer")
    .about("Reads a file and prints its content")
    .arg(Arg::new("write")
        .long("write")
        .value_name("FILE")
        .help("Path to file to write"))
    .arg(Arg::new("read")
        .long("read")
        .value_name("FILE")
        .help("Path to file to read"))
    .get_matches();

    if let Some(write_path) = matches.get_one::<String>("write") {
        if let Err(error) = write_to_file(write_path) {
            handle_write_error(error);
        }
    }

    if let Some(read_path) = matches.get_one::<String>("read") {
        if let Err(error) = read_file(read_path) {
            handle_read_error(error);
        }
    }
}