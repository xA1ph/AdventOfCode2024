use regex::{self, Regex};
use std::{fs, path::Path};

fn main() {
    let path = Path::new("data.txt");
    #[cfg(test)]
    let path = Path::new("test.txt");

    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let re = Regex::new(r"mul\((?<left>[0-9]+)\,(?<right>[0-9]+)\)").unwrap();

    let result: u32 = re
        .captures_iter(contents.as_str())
        .map(|caps| {
            (
                caps["left"].parse::<u32>().unwrap(),
                caps["right"].parse::<u32>().unwrap(),
            )
        })
        .map(|(l, r)| l * r)
        .sum();

    println!("result is: {}", result);
}
