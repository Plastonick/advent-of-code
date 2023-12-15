use std::fs;

pub fn get_file_contents(file: &str) -> String {
    let file_path = format!("src/inputs/{file}.input");
    let contents = fs::read_to_string(file_path.clone()).expect(&format!(
        "Should have been able to read the file {file_path}"
    ));

    contents
}

pub fn get_lines(file: &str) -> Vec<String> {
    let contents = get_file_contents(&file);

    contents
        .lines()
        .map(|x| String::from(x))
        .collect::<Vec<String>>()
}
