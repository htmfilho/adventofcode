pub mod utils {
    use std::fs;

    pub fn read_input_file(path: &str) -> Vec<String> {
        fs::read_to_string(path)
            .unwrap()
            .split('\n')
            .into_iter()
            .map(String::from)
            .collect()
    }
}