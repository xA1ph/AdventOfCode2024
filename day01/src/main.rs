use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("data.txt");

    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let (mut l1, mut l2): (Vec<_>, Vec<_>) = contents
        .split("\n")
        .map(|s| {
            let s1: i32 = s.get(0..5).unwrap().parse().unwrap();
            let s2: i32 = s.get(8..13).unwrap().parse().unwrap();
            (s1, s2)
        })
        .unzip();

    l1.sort();
    l2.sort();

    let result: i32 = l1
        .iter()
        .zip(l2.iter())
        .map(|(v1, v2)| (v1 - v2).abs())
        .sum();

    println!("total difference is: {}", result);

    let result: i32 = l1
        .iter()
        .map(|v1| l2.iter().filter(|&v2| v2 == v1).count() as i32 * v1)
        .sum();

    println!("similarity score is: {}", result);
}
