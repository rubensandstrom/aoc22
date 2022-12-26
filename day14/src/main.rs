use std::{fs, collections::HashSet};

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .split(" -> ")
        .map(|s| {
            if let Some(pair) = s.split_once(",") {
                let start = pair.0.parse().unwrap();
                let stop = pair.1.parse().unwrap();
                (start, stop)
            } else {(0,0)}
        })
        .collect()
        
}

fn make_map(input: &str) -> (HashSet<(usize, usize)>, usize) {

    let mut v = vec![];
    for line in input.lines() {
        v.push(parse_input(line));

    }

    let mut floor = 0;
    for i in &v {
        for j in i {
            if j.1 > floor { floor = j.1; }
        }
    }

    let mut map: HashSet<(usize, usize)> = HashSet::new();

    for i in &v {
        for j in 0..i.len()-1 {
            let a = i[j];
            let b = i[j+1];

            if a.0 == b.0 {
                if a.1 < b.1 {
                    for k in a.1..=b.1 { map.insert((a.0, k)); }
                } else if b.1 < a.1 {
                    for k in b.1..=a.1 { map.insert((a.0, k)); }
                }
            } else if a.1 == b.1 {
                if a.0 < b.0 {
                    for k in a.0..=b.0 { map.insert((k, a.1)); }
                } else if b.0 < a.0 {
                    for k in b.0..=a.0 { map.insert((k, a.1)); }
                }
            }
            
        }
    }

    return (map, floor);
}

fn task1(input: &str) -> i32 {
    let (mut map, floor) = make_map(&input);

    let mut size = 0;
    let (mut x, mut y) = (500, 0);

    loop {

        if y > floor  { 
            break;
        }

        if !map.contains(&(x, y+1)) { 
            y += 1; 
            continue;
        } else if !map.contains(&(x-1, y+1)) { 
            y += 1;
            x -= 1;
            continue;
        } else if !map.contains(&(x+1, y+1)) {
            y += 1;
            x += 1;
            continue;
        } else {
            map.insert((x, y));
            size += 1;
            (x, y) = (500, 0);
        }
    }
    return size;
}

fn task2(input: &str) -> i32 {
    let (mut map, floor) = make_map(&input);

    let mut size = 0;
    let (mut x, mut y) = (500, 0);

    loop {

        if map.contains(&(x, y)) {
            break;
        }
        if y > floor  { 
            map.insert((x, y));
            size += 1;
            (x, y) = (500, 0);
            continue;
        }

        if !map.contains(&(x, y+1)) { 
            y += 1; 
            continue;
        } else if !map.contains(&(x-1, y+1)) { 
            y += 1;
            x -= 1;
            continue;
        } else if !map.contains(&(x+1, y+1)) {
            y += 1;
            x += 1;
            continue;
        } else {
            map.insert((x, y));
            size += 1;
            (x, y) = (500, 0);
        }
    }
    return size;
}

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file!");

    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

#[cfg(test)]
mod test {
    use super::{task1, task2};
    use std::fs;

    #[test]
    fn test1() {
        let test = fs::read_to_string("test").expect("Couldn't read file!");
        assert_eq!(task1(&test), 24);
    }
    #[test]
    fn test2() {
        let test = fs::read_to_string("test").expect("Couldn't read file!");
        assert_eq!(task2(&test), 93);
    }
}
