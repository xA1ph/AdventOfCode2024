use regex::{self, Regex};
use std::{fs, path::Path};

fn apply_re(re: &Regex, s: &str) -> u32 {
    re.captures_iter(s)
        .map(|caps| {
            (
                caps["left"].parse::<u32>().unwrap(),
                caps["right"].parse::<u32>().unwrap(),
            )
        })
        .map(|(l, r)| l * r)
        .sum()
}

fn cleanse_string(s: &String) -> String {
    s.split("do()")
        .map(|v| match v.split_once("don't()") {
            Some(v) => v.0,
            None => v,
        })
        .collect()
}

fn main() {
    let path = Path::new("data.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let re = Regex::new(r"mul\((?<left>[0-9]+)\,(?<right>[0-9]+)\)").unwrap();

    println!("result is: {}", apply_re(&re, &contents));

    println!(
        "more optimized result is: {}",
        apply_re(&re, &cleanse_string(&contents))
    );
}
