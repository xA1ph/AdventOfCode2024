use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("data.txt");
    let contents = fs::read_to_string(path).unwrap();
    let map: Vec<&str> = contents.lines().filter(|&l| l != "").collect();

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

    let result = calculate_antinodes(&antenna_map, row_cnt, col_cnt, false);

    println!("Numer of unique antinode locations: {}", result.len());

    let result = calculate_antinodes(&antenna_map, row_cnt, col_cnt, true);

    println!(
        "Numer of unique antinode locations considering resonant harmonics: {}",
        result.len()
    );
}

fn calculate_antinodes(
    antenna_map: &HashMap<char, HashSet<(i32, i32)>>,
    row_cnt: usize,
    col_cnt: usize,
    resonance: bool,
) -> HashSet<(i32, i32)> {
    let mut result: HashSet<(i32, i32)> = HashSet::new();
    antenna_map.iter().for_each(|(_, set)| {
        let vec: Vec<_> = set.iter().collect();
        for i in 0..vec.len() {
            let &e1 = vec.get(i).unwrap();
            for j in 0..vec.len() {
                if j == i {
                    continue;
                }
                let &e2 = vec.get(j).unwrap();
                let x_diff = e1.0 - e2.0;
                let y_diff = e1.1 - e2.1;
                let mut start = *e1;
                if resonance {
                    result.insert(start);
                }
                loop {
                    let antinode = (start.0 + x_diff, start.1 + y_diff);
                    if antinode.0 >= 0
                        && antinode.0 < col_cnt as i32
                        && antinode.1 >= 0
                        && antinode.1 < row_cnt as i32
                    {
                        result.insert(antinode);
                    } else {
                        break;
                    }

                    if !resonance {
                        break;
                    }
                    start = antinode;
                }
            }
        }
    });
    result
}
