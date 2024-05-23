use std::env;

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabyte(f64),
}
impl FileSize {
    fn format_size(size: u64) -> String {
        let filesize = match size {
            0..=999 => FileSize::Bytes(size),
            1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
            1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
            1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
            _ => FileSize::Terabyte(size as f64 / 1_000_000_000_000.0 ),
        };

        match filesize {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
            FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
            FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
            FileSize::Terabyte(tb) => format!("{:.2} TB", tb),
        }
    }
}



fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} <size in bytes>", args[0]);
        return;
    }
    let size = args[1].parse::<u64>().unwrap();
    let result = FileSize::format_size(size);
    println!("{}", result)
}