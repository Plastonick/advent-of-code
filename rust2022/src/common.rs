use std::fs;

pub fn get_file_contents(file: &str) -> String {
    let file_path = format!("src/inputs/{file}.txt");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    contents
}

pub fn get_lines(file: &str) -> Vec<String> {
    let contents = get_file_contents(&file);

    contents
        .lines()
        .map(|x| String::from(x))
        .collect::<Vec<String>>()
}
