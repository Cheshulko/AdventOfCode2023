use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Result<Vec<String>, std::io::Error> {
    Ok(read_to_string(filename)?
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>())
}
