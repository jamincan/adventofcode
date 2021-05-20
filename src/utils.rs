use std::io;

pub type AocResult = std::result::Result<i64, Box<dyn std::error::Error>>;

pub fn load(year: u16, day: u8) -> io::Result<String> {
    let path = format!("data/{}/day{}.txt", year, day);
    std::fs::read_to_string(path)
}
