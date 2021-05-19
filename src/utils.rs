use std::io;

pub fn load(year: u16, day: u8) -> io::Result<String> {
    let path = format!("data/{}/day{}.txt", year, day);
    std::fs::read_to_string(path)
}
