pub mod utils {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    pub fn get_lines(file_name: &str) -> Vec<String> {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            lines.push(line.unwrap());
        }

        return lines;
    }

    pub fn get_ints(file_name: &str) -> Vec<i32> {
        let lines = get_lines(file_name);
        let values = lines
            .iter()
            .map(|line| line.parse::<i32>().unwrap())
            .collect();

        return values;
    }
}
