use std::fs;

pub fn get_lines(file: &str) -> Vec<String> {
    let file_path = format!("src/inputs/{file}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    contents
        .lines()
        .map(|x| String::from(x))
        .collect::<Vec<String>>()
}