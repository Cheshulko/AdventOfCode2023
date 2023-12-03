use std::fs::read_to_string;

pub type Res<T> = core::result::Result<T, Box<dyn std::error::Error>>;

pub fn read_lines(filename: &str) -> Res<Vec<String>> {
    Ok(read_to_string(filename)?
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>())
}
