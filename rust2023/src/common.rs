use std::fs;

pub type Answer = (String, String);

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

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn rotate_90<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    transpose(v)
        .into_iter()
        .map(|mut r| {
            r.reverse();
            r
        })
        .collect()
}

pub fn rotate_270<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    transpose(
        v.into_iter()
            .map(|mut r| {
                r.reverse();
                r
            })
            .collect(),
    )
}
