use std::env;

#[derive(Debug)]
struct FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}
impl FileSize {
    fn format_size(size: u64, unit: &str) -> Self {
        // let filesize = match size {
        //     0..=999 => FileSize::Bytes(size),
        //     1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        //     1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        //     _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
        // };
    
        match String::from(unit).to_lower() {
            "bytes" => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
            FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
            FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
        }
    }
}



fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        println!("Usage: {} <size bytes | KB | MB | GB >", args[0]);
        return;
    }
    
    println!("{}", result)
}