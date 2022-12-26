use std::{fs, collections::HashMap, usize};

type Coord = (usize, usize);

fn parse ( input: &str ) -> (Vec<Vec<usize>>, Coord, Coord) {

    let mut start: Coord = (0, 0);
    let mut goal: Coord = (0, 0);

    let mut parsed_input = vec![];
    for (row, l) in input.lines().enumerate() {
        let mut v = vec![];
        for (col, c) in l.chars().enumerate() {
            if c == 'S' { 
                start = (row, col); 
                v.push(0);
                continue;
            }
            if c == 'E' { 
                goal = (row, col); 
                v.push('z' as usize - 'a' as usize);
                continue;
            }
            v.push(c as usize - 'a' as usize);
        }
        parsed_input.push(v);
    }
    return (parsed_input, start, goal);
}

fn bfs( input: Vec<Vec<usize>>, start: Vec<Coord>, goal: Coord ) -> usize {

    let mut visited: HashMap<Coord, usize> = HashMap::new();
    let mut to_visit = vec![];

    for s in start {
        visited.insert(s, 0);
        to_visit.push(s);
    }

    while !(to_visit.is_empty()) {
        let (row, col) = to_visit.remove(0);
        let depth = visited[&(row, col)];
        if (row, col) == goal { return depth; }

        for (d_row, d_col) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let new_row = row as i32 + d_row;
            let new_col = col as i32 + d_col;

            if new_row < 0 || new_row > (input.len() as i32) - 1 ||
                new_col < 0 || new_col > (input[0].len() as i32) - 1
            { 
                continue; 
            }
            if !visited.contains_key(&(new_row as usize, new_col as usize)) {
                if input[new_row as usize][new_col as usize] as i32 - input[row][col] as i32  <= 1 {

                    visited.insert((new_row as usize, new_col as usize), depth+1);
                    to_visit.push((new_row as usize, new_col as usize));
                }
            }
        }
    }
    return 0;
}

fn task1(input: &str) -> usize {

    let (input, start, goal) = parse(input);
    let path = bfs(input.clone(), vec![start], goal);

    return path;
}
fn task2(input: &str) -> usize {

    let (input, _start, goal) = parse(input);
    let mut lo = vec![];
    
    for row in 0..input.len() {
        for col in 0..input[0].len() { 
            if input[row][col] == 0 {
                lo.push((row, col));
            }
        }
    }

    return bfs(input, lo, goal);
}

fn main() {

    let input = fs::read_to_string("input").expect("Couldn't read file!");
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

#[cfg(test)]
mod test {
    use super::{ task1, task2 };
    use std::fs;

    #[test]
    fn test1() {
        let test = fs::read_to_string("test").expect("Couldn't read file!");
        assert_eq!(task1(&test), 31);
    }
    #[test]
    fn test2() {
        let test = fs::read_to_string("test").expect("Couldn't read file!");
        assert_eq!(task2(&test), 29);
    }
}
