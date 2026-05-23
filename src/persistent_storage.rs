use std::fs::OpenOptions;
use std::io::Write;

pub fn add_data_toa_file(command: String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("plotwhole.txt")
        .unwrap();
    writeln!(file, "{command}").unwrap();
}
