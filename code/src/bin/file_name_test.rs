use matura::increment_last_number_in_filename;

fn main() {
    let file_paths = [
        "./cal/cal1.png",
        "./cal/cal10.png",
        "./cal/cal1.11_29.png",
        "./cal/cal1.11_29.1.png",
    ];

    for file_path in &file_paths {
        match increment_last_number_in_filename(file_path) {
            Some(new_path) => println!("{} -> {}", file_path, new_path),
            None => println!("Failed to process {}", file_path),
        }
    }
}
