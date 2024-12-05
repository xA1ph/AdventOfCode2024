use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("data.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    //i hate this but split_once("\n\n") doesnt work for some reason :(
    let idx = contents.lines().position(|s| s == "").unwrap();
    let rules: Vec<_> = contents.lines().take(idx).collect();
    let pages: Vec<_> = contents.lines().skip(idx + 1).collect();

    let rules: Vec<_> = rules
        .iter()
        .map(|&s| s.split_once("|").unwrap())
        .map(|(s1, s2)| (s1.parse::<u32>().unwrap(), s2.parse::<u32>().unwrap()))
        .collect();
    let pages: Vec<_> = pages
        .iter()
        .map(|&s| {
            s.split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let result: u32 = pages
        .iter()
        .filter(|&p| check_page(p, &rules))
        .map(|v| v.get(v.len() / 2).unwrap())
        .sum();

    println!("sum of correctly ordered middle page numbers: {}", result);

    let result: u32 = pages
        .iter()
        .filter(|&p| !check_page(p, &rules))
        .map(|p| order_page(&p, &rules))
        .map(|v| v[v.len() / 2])
        .sum();

    println!("sum of incorrectly ordered middle page numbers: {}", result);
}

fn order_page(page: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for p in page {
        let prevs: Vec<_> = rules
            .iter()
            .filter(|(_, v2)| *v2 == *p)
            .map(|(v, _)| *v)
            .collect();
        let mut idx = None;
        for i in 0..result.len() {
            if prevs.contains(&result[i]) {
                idx = Some(i);
            }
        }
        match idx {
            None => result.insert(0, *p),
            Some(i) => result.insert(i + 1, *p),
        }
    }
    result
}

fn check_page(page: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    for i in 0..page.len() {
        let cur = page[i];
        let nexts = &page[(i + 1)..];
        let prevs: Vec<_> = rules
            .iter()
            .filter(|(_, v2)| *v2 == cur)
            .map(|(v, _)| *v)
            .collect();
        for n in nexts {
            if prevs.contains(n) {
                return false;
            }
        }
    }
    true
}
