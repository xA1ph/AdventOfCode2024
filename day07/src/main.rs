use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("data.txt");
    let contents = fs::read_to_string(path).unwrap();

    let contents: Vec<_> = contents.lines().map(|s| {
        let Some((res, data)) = s.split_once(":") else { panic!("Error parsing input data") };
        let data: Vec<u64> = data.trim().split(" ").map(|s| s.parse().unwrap()).collect();
        (res.parse::<u64>().unwrap(), data)
    }).collect();

    let result: u64 = contents.iter().filter(|&e| check_calibration(e, false)).map(|(res, _)| res).sum();

    println!("total sum of calibration results: {}", result);

    let result: u64 = contents.iter().filter(|&e| check_calibration(e, true)).map(|(res, _)| res).sum();

    println!("total sum of calibration results considering the '||' operator: {}", result);
}

fn check_calibration(data: &(u64, Vec<u64>), concat: bool) -> bool {
    let res = data.0;
    let acc = data.1.first().unwrap().clone();
    let mut data = data.1.clone();
    data.remove(0);

    check_calibration_rec(res, acc, &data, concat)
}

fn check_calibration_rec(res: u64, acc: u64, data: &Vec<u64>, concat: bool) -> bool {
    if data.is_empty() {
        return if res == acc {
            true
        } else {
            false
        };
    }

    let first = data.first().unwrap();
    let data = &data[1..data.len()].to_vec();

    if !concat {
        check_calibration_rec(res, acc * first, data, false) || check_calibration_rec(res, acc + first, data, false)
    } else {
        let mut acc_concat = acc.to_string();
        acc_concat.push_str(first.to_string().as_str());
        let acc_concat: u64 = acc_concat.parse().unwrap();
        check_calibration_rec(res, acc * first, data, true) || check_calibration_rec(res, acc + first, data, true) || check_calibration_rec(res, acc_concat, data, true)
    }
}
