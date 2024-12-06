use std::{collections::HashSet, fs, path::Path};

struct Guard {
    pos: (i32, i32),
    direction: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl Guard {
    const NORTH: (i32, i32) = (0, -1);
    const SOUTH: (i32, i32) = (0, 1);
    const EAST: (i32, i32) = (1, 0);
    const WEST: (i32, i32) = (-1, 0);

    fn new(pos: (i32, i32)) -> Self {
        let visited: HashSet<(i32, i32)> = HashSet::new();
        Guard {
            pos,
            direction: Self::NORTH,
            visited,
        }
    }

    fn step(&mut self, map: &Vec<&str>) -> bool {
        let cur = self.pos;
        let next = (cur.0 + self.direction.0, cur.1 + self.direction.1);

        if next.0 < 0
            || next.0 as usize >= map[cur.1 as usize].len()
            || next.1 < 0
            || next.1 as usize >= map.len()
        {
            self.visited.insert(cur);
            return false;
        }

        if map[next.1 as usize].chars().nth(next.0 as usize).unwrap() == '#' {
            self.direction = match self.direction {
                Guard::NORTH => Guard::EAST,
                Guard::EAST => Guard::SOUTH,
                Guard::SOUTH => Guard::WEST,
                Guard::WEST => Guard::NORTH,
                _ => panic!("currently facing invalid direction!"),
            }
        } else {
            self.visited.insert(cur);
            self.pos = next;
        }
        true
    }
}

fn main() {
    let path = Path::new("data.txt");
    let contents = fs::read_to_string(path).unwrap();

    let map: Vec<&str> = contents.split("\n").filter(|&s| s != "").collect();

    println!("len: {}", map.len());

    let pos = (
        map.iter()
            .find(|r| r.contains("^"))
            .unwrap()
            .chars()
            .position(|c| c == '^')
            .unwrap() as i32,
        map.iter().position(|r| r.contains("^")).unwrap() as i32,
    );

    println!("pos: {:?}", pos);

    let mut guard = Guard::new(pos);

    while guard.step(&map) {}

    let result = guard.visited.len();

    println!("Different locations Guard has been to: {}", result);
}
