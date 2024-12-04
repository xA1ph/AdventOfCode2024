use std::fs;
use std::path::Path;

static LENGTH: usize = 140; //characters in one line

struct Direction(i32, i32);

impl Direction {
    const UP: Self = Self(0, -1);
    const DOWN: Self = Self(0, 1);
    const LEFT: Self = Self(-1, 0);
    const RIGHT: Self = Self(1, 0);
    const UP_LEFT: Self = Self(-1, -1);
    const UP_RIGHT: Self = Self(1, -1);
    const DOWN_LEFT: Self = Self(-1, 1);
    const DOWN_RIGHT: Self = Self(1, 1);

    const DIRECTIONS: [Self; 8] = [
        Self::UP,
        Self::DOWN,
        Self::LEFT,
        Self::RIGHT,
        Self::UP_LEFT,
        Self::UP_RIGHT,
        Self::DOWN_LEFT,
        Self::DOWN_RIGHT,
    ];

    fn get(pos: &(usize, usize), dir: &Self) -> Option<(usize, usize)> {
        match (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1) {
            (x, y) if x < 0 || y < 0 => None,
            (x, y) if x >= LENGTH as i32 || y >= LENGTH as i32 => None,
            (x, y) => Some((x as usize, y as usize)),
        }
    }
}

fn find_xmas(data: &[&str], row: usize, col: usize) -> usize {
    let xmas = "XMAS".to_string();
    let mut result = 0;
    for i in Direction::DIRECTIONS {
        if xmas.chars().nth(0) != data[row].chars().nth(col) {
            break;
        }
        let mut tmp_row = row;
        let mut tmp_col = col;
        let mut found = true;
        for j in 1..xmas.len() {
            let Some((x, y)) = Direction::get(&(tmp_row, tmp_col), &i) else {
                found = false;
                break;
            };
            (tmp_row, tmp_col) = (x, y);
            if xmas.chars().nth(j) != data[tmp_row].chars().nth(tmp_col) {
                found = false;
                break;
            }
        }
        if found {
            result += 1;
        }
    }
    result
}

fn find_x_mas(data: &[&str], row: usize, col: usize) -> usize {
    if data[row].chars().nth(col) == Some('A') {
        //println!("found A");
        let (Some(ur), Some(ul), Some(dr), Some(dl)) = (
            Direction::get(&(row, col), &Direction::UP_RIGHT),
            Direction::get(&(row, col), &Direction::UP_LEFT),
            Direction::get(&(row, col), &Direction::DOWN_RIGHT),
            Direction::get(&(row, col), &Direction::DOWN_LEFT),
        ) else {
            return 0;
        };

        let (ur, ul, dr, dl) = (
            data[ur.0].chars().nth(ur.1).unwrap(),
            data[ul.0].chars().nth(ul.1).unwrap(),
            data[dr.0].chars().nth(dr.1).unwrap(),
            data[dl.0].chars().nth(dl.1).unwrap(),
        );

        if vec![ur, ul, dr, dl]
            .iter()
            .find(|&c| *c != 'M' && *c != 'S')
            .is_some()
        {
            return 0;
        }

        if ur == dl || ul == dr {
            return 0;
        }

        return 1;
    }
    0
}

fn main() {
    let path = Path::new("data.txt");

    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let contents: Vec<_> = contents.split("\n").collect();

    let mut result = 0;
    for i in 0..contents.len() {
        for j in 0..contents[i].len() {
            result += find_xmas(&contents, i, j);
        }
    }

    println!("Count of XMAS: {}", result);

    let mut result = 0;
    for i in 0..contents.len() {
        for j in 0..contents[i].len() {
            result += find_x_mas(&contents, i, j);
        }
    }

    println!("Count of X-MAS: {}", result);
}
