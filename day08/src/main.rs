use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("test.txt");
    let contents = fs::read_to_string(path).unwrap();
    let map: Vec<&str> = contents.lines().collect();

    let mut antenna_map: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();

    map.iter().enumerate().for_each(|(y, &s)| {
        s.chars().enumerate().for_each(|(x, c)| {
            if c == '.' {
                return;
            }
            match antenna_map.get_mut(&c) {
                None => {
                    let _ = antenna_map.insert(c, HashSet::from([(x as i32, y as i32)]));
                }
                Some(val) => {
                    let _ = val.insert((x as i32, y as i32));
                }
            };
        })
    });

    let row_cnt = map.len();
    let col_cnt = map.first().unwrap().len();

    let result = calculate_antinodes(&antenna_map, row_cnt, col_cnt);
    println!("{:?}", result);

    println!("Numer of unique antinode locations: {}", result.len());
}

fn calculate_antinodes(
    antenna_map: &HashMap<char, HashSet<(i32, i32)>>,
    row_cnt: usize,
    col_cnt: usize,
) -> HashSet<(i32, i32)> {
    let mut result: HashSet<(i32, i32)> = HashSet::new();
    antenna_map.iter().for_each(|(_, set)| {
        let vec: Vec<_> = set.iter().collect();
        for i in 0..vec.len() {
            let &e1 = vec.get(i).unwrap();
            for &e2 in &vec[i + 1..vec.len()] {
                let x_diff = min(e1.0, e2.0) - e1.0.abs_diff(e2.0) as i32;
                let y_diff = min(e1.1, e2.1) - e1.1.abs_diff(e2.1) as i32;
                let mut a1 = e1.clone();
                let mut a2 = e2.clone();
                if e1.0 > e2.0 {
                    a1.0 += x_diff;
                    a2.0 -= x_diff;
                } else {
                    a1.0 -= x_diff;
                    a2.0 += x_diff;
                }
                if e1.1 > e2.1 {
                    a1.1 += y_diff;
                    a2.1 -= y_diff;
                } else {
                    a1.1 -= y_diff;
                    a2.1 += y_diff;
                }
                if a1.0 >= 0 && a1.0 < col_cnt as i32 && a1.1 >= 0 && a1.1 < row_cnt as i32 {
                    result.insert(a1);
                }
                if a2.0 >= 0 && a2.0 < col_cnt as i32 && a2.1 >= 0 && a2.1 < row_cnt as i32 {
                    result.insert(a2);
                }
            }
        }
    });
    result
}
