use std::{fs, path::Path};

fn main() {
    let path = Path::new("data.txt");
    let contents = fs::read_to_string(path).unwrap();
    let map: Vec<&str> = contents.lines().filter(|&l| l != "").collect();

    let mut starts = Vec::new();
    map.iter().enumerate().for_each(|(y, vec)| {
        vec.chars().enumerate().for_each(|(x, c)| {
            if c == '0' {
                starts.push((x as i16, y as i16));
            }
        })
    });

    let result: usize = starts
        .iter()
        .map(|start| {
            let mut ends = Vec::new();
            calculate_trailhead_score_ends(&map, start, &mut ends)
        })
        .sum();
    println!("sum of all trailhead scores: {}", result);

    let result: usize = starts
        .iter()
        .map(|start| calculate_trailhead_score(&map, start))
        .sum();
    println!("sum of all trailhead ratings: {}", result);
}

fn calculate_trailhead_score_ends(
    map: &Vec<&str>,
    start: &(i16, i16),
    ends: &mut Vec<(i16, i16)>,
) -> usize {
    calculate_trailhead_score_ends_rec(&map, &(start.0 + 1, start.1), 0, ends)
        + calculate_trailhead_score_ends_rec(&map, &(start.0 - 1, start.1), 0, ends)
        + calculate_trailhead_score_ends_rec(&map, &(start.0, start.1 + 1), 0, ends)
        + calculate_trailhead_score_ends_rec(&map, &(start.0, start.1 - 1), 0, ends)
}

fn calculate_trailhead_score_ends_rec(
    map: &Vec<&str>,
    start: &(i16, i16),
    prev: usize,
    ends: &mut Vec<(i16, i16)>,
) -> usize {
    //println!("called fn with start: {:?}", start);
    let Some(row) = map.get(start.1 as usize) else {
        return 0;
    };
    let Some(elem) = row.chars().nth(start.0 as usize) else {
        return 0;
    };
    let current: usize = elem.to_digit(10).unwrap() as usize;

    if prev == 8 && current == 9 && !ends.contains(&(start.0, start.1)) {
        ends.push((start.0, start.1));
        return 1;
    }

    return if current == prev + 1 {
        calculate_trailhead_score_ends_rec(&map, &(start.0 + 1, start.1), current, ends)
            + calculate_trailhead_score_ends_rec(&map, &(start.0 - 1, start.1), current, ends)
            + calculate_trailhead_score_ends_rec(&map, &(start.0, start.1 + 1), current, ends)
            + calculate_trailhead_score_ends_rec(&map, &(start.0, start.1 - 1), current, ends)
    } else {
        0
    };
}

fn calculate_trailhead_score(map: &Vec<&str>, start: &(i16, i16)) -> usize {
    calculate_trailhead_score_rec(&map, &(start.0 + 1, start.1), 0)
        + calculate_trailhead_score_rec(&map, &(start.0 - 1, start.1), 0)
        + calculate_trailhead_score_rec(&map, &(start.0, start.1 + 1), 0)
        + calculate_trailhead_score_rec(&map, &(start.0, start.1 - 1), 0)
}

fn calculate_trailhead_score_rec(map: &Vec<&str>, start: &(i16, i16), prev: usize) -> usize {
    //println!("called fn with start: {:?}", start);
    let Some(row) = map.get(start.1 as usize) else {
        return 0;
    };
    let Some(elem) = row.chars().nth(start.0 as usize) else {
        return 0;
    };
    let current: usize = elem.to_digit(10).unwrap() as usize;

    if prev == 8 && current == 9 {
        return 1;
    }

    return if current == prev + 1 {
        calculate_trailhead_score_rec(&map, &(start.0 + 1, start.1), current)
            + calculate_trailhead_score_rec(&map, &(start.0 - 1, start.1), current)
            + calculate_trailhead_score_rec(&map, &(start.0, start.1 + 1), current)
            + calculate_trailhead_score_rec(&map, &(start.0, start.1 - 1), current)
    } else {
        0
    };
}
