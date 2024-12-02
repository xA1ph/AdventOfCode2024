use std::{fs, path::Path};

fn is_inc(v: &Vec<u32>) -> bool {
    let mut v2 = v.clone();
    v2.sort();
    *v == v2
}

fn is_dec(v: &Vec<u32>) -> bool {
    let mut v = v.clone();
    v.reverse();
    is_inc(&v)
}

fn check_diff(v: &Vec<u32>) -> bool {
    let Some(mut cur) = v.first() else {
        return false;
    };
    for e in v[1..v.len()].iter() {
        let diff = cur.abs_diff(*e);
        if diff == 0 || diff > 3 {
            return false;
        }
        cur = e;
    }
    true
}

fn is_safe(v: &Vec<u32>) -> bool {
    (is_inc(v) || is_dec(v)) && check_diff(v)
}

fn is_almost_safe(v: &Vec<u32>) -> bool {
    let mut permut = Vec::new();
    (0..v.len()).for_each(|i| {
        permut.push({
            let mut to_insert = v.clone();
            to_insert.remove(i);
            to_insert
        })
    });
    permut.iter().filter(|v| is_safe(v)).count() > 0
}

fn main() {
    let path = Path::new("data.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let contents: Vec<_> = contents
        .split("\n")
        .map(|l| {
            l.split(" ")
                .map(|n| n.trim().parse::<u32>().ok())
                .flatten()
                .collect::<Vec<u32>>()
        })
        .collect();

    let result = contents.iter().filter(|v| is_safe(v)).count();

    println!("number of safe reports: {}", result);

    let result = contents.iter().filter(|v| is_almost_safe(v)).count();

    println!("number of single level bad safe reports: {}", result);
}
