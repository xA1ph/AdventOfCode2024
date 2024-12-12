use std::{
    collections::{HashSet, VecDeque},
    fs,
    path::Path,
};

fn main() {
    let path = Path::new("data.txt");
    let contents = fs::read_to_string(path).unwrap();
    let contents = contents.trim();

    let disk_map: Vec<_> = contents.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mem = compress_fs(&disk_map);
    let result: usize = hash_mem(&mem);

    println!("Filesystem checksum: {}", result);

    let mem = compress_fs_blocks(&disk_map);
    let result: usize = hash_mem(&mem);

    println!("Filesystem checksum, when moving whole files: {}", result);
}

fn hash_mem(mem: &Vec<usize>) -> usize {
    mem.iter().enumerate().map(|(idx, val)| idx * val).sum()
}

fn compress_fs(disk_map: &Vec<u32>) -> Vec<usize> {
    let mut used: VecDeque<_> = disk_map
        .iter()
        .step_by(2)
        .enumerate()
        .map(|(idx, n)| {
            let mut v = Vec::new();
            for _ in 0..*n as usize {
                v.push(idx);
            }
            v
        })
        .flatten()
        .collect();

    let mut mem: Vec<usize> = Vec::new();
    let mut front = true;
    for i in disk_map {
        for _ in 0..*i as usize {
            if let Some(val) = if front {
                used.pop_front()
            } else {
                used.pop_back()
            } {
                mem.push(val);
            } else {
                break;
            }
        }
        front = !front
    }
    mem
}

fn compress_fs_blocks(disk_map: &Vec<u32>) -> Vec<usize> {
    let mut allocated: VecDeque<_> = disk_map
        .iter()
        .step_by(2)
        .enumerate()
        .map(|(idx, n)| {
            let mut v = Vec::new();
            for _ in 0..*n as usize {
                v.push(idx);
            }
            v
        })
        .collect();

    let mut free: VecDeque<_> = disk_map
        .iter()
        .skip(1)
        .step_by(2)
        .map(|n| {
            let mut v = Vec::new();
            for _ in 0..*n as usize {
                v.push(0);
            }
            (*n, v)
        })
        .collect();

    //println!("{:?}", free);

    let allocated_rev: VecDeque<_> = allocated.clone();
    let allocated_rev: VecDeque<_> = allocated_rev.iter().rev().collect();

    for v in allocated_rev {
        let Some(pos) = free.iter().position(|(size, _)| *size >= v.len() as u32) else {
            continue;
        };
        let old = free.get_mut(pos).unwrap();
        old.0 = old.0 - v.len() as u32;
        old.1.truncate(old.0 as usize);
        free.insert(pos, (0, v.clone()));
    }

    let mut free: VecDeque<_> = free
        .iter()
        .filter(|(_, val)| !val.is_empty())
        .map(|(_, v)| v.clone())
        .collect();

    //println!("{:?}", free);

    let mut result = Vec::new();
    let mut even = true;

    for n in disk_map {
        if even {
            let Some(mut value) = allocated.pop_front() else {
                break;
            };
            //let size = value.len();
            //let Some(id) = value.first() else {
            //    panic!("Impossible");
            //};
            result.append(&mut value);
        } else {
            let mut reps = *n;
            while reps != 0 {
                let Some(mut value) = free.pop_front() else {
                    break;
                };
                let size = value.len();
                //let id = value.first().unwrap();

                result.append(&mut value);

                reps -= size as u32;
            }
        }

        even = !even
    }

    //println!("{:?}", result);

    let mut indicies: HashSet<usize> = HashSet::new();
    let mut sum = 0;
    for l in disk_map {
        //println!("{}", *l);
        let subject = &mut result[sum..sum + *l as usize];
        for i in 0..subject.len() {
            if indicies.contains(&subject[i]) {
                subject[i] = 0;
            }
        }
        for elem in subject {
            indicies.insert(*elem);
        }
        sum += *l as usize;
    }

    //println!("{:?}", result);

    result
}
