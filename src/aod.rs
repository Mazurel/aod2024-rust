use std::{fs::File, io::Read};

pub type SResult<T> = Result<T, String>;

pub fn solution_input_to_list_of_strings(input_path: &str) -> SResult<Vec<String>> {
    let mut result = String::new();
    File::open(input_path)
        .map_err(|err| format!("{err}"))?
        .read_to_string(&mut result)
        .map_err(|err| format!("{err}"))?;

    Ok(result
        .split("\n")
        .map(|substr| substr.to_string())
        .filter(|substr| substr.len() > 0)
        .collect())
}
