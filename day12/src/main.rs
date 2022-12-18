use std::{fs, collections::HashMap, usize};

type Coord = (usize, usize);

fn parse ( input: &str ) -> (Vec<Vec<i32>>, Coord, Coord) {

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
                v.push('z' as i32 - 'a' as i32);
                continue;
            }
            v.push(c as i32 - 'a' as i32);
        }
        parsed_input.push(v);
    }
    return (parsed_input, start, goal);
}

fn bfs ( input: Vec<Vec<i32>>, start: Coord, goal: Coord ) -> Vec<Coord> {

    let mut visited: HashMap<Coord, Vec<Coord>> = HashMap::new();
    let mut to_visit = vec![]; // might change to que if grows to big

    visited.insert(start, vec![]);
    to_visit.push(start);

    while !to_visit.is_empty() {
        let (row, col) = to_visit.remove(0); 
        let path = visited[&(row, col)].clone();
        if (row, col) == goal { return path.clone(); }

        if row > 0 {
            if !visited.contains_key(&(row-1, col)) {
                if input[row-1][col] - input[row][col] <= 1 {

                    visited.insert((row-1, col), [path.clone(), vec![(row-1, col)]].concat());
                    to_visit.push((row-1, col));
                }
            }
        }
        if col > 0 {
            if !visited.contains_key(&(row, col-1)) {
                if input[row][col-1] - input[row][col] <= 1 {

                    visited.insert((row, col-1), [path.clone(), vec![(row, col-1)]].concat());
                    to_visit.push((row, col-1));
                }
            }
        }
        if row < input.len()-1 {
            if !visited.contains_key(&(row+1, col)) {
                if input[row+1][col] - input[row][col] <= 1 {

                    visited.insert((row+1, col), [path.clone(), vec![(row+1, col)]].concat());
                    to_visit.push((row+1, col));
                }
            }
        }
        if col < input[0].len()-1 {
            if !visited.contains_key(&(row, col+1)) {
                if input[row][col+1] - input[row][col] <= 1 {

                    visited.insert((row, col+1), [path.clone(), vec![(row, col+1)]].concat());
                    to_visit.push((row, col+1));
                }
            }
        }

    }
    return vec![];
}

fn task1(input: &str) -> i32 {

    let (input, start, goal) = parse(input);
    let path = bfs(input.clone(), start, goal);

    return path.len() as i32;
}
fn task2(input: &str) -> i32 {

    let (input, _start, goal) = parse(input);
    let mut lo = vec![];
    
    for row in 0..input.len() {
        for col in 0..input[0].len() { 
            if input[row][col] == 0 {
                lo.push((row, col));
            }
        }
    }

    let mut routes = vec![];
    for a in lo {
        let route = bfs(input.clone(), a, goal).len();
        if route != 0 { routes.push(route); }
    }

    routes.sort();
    return routes[0] as i32;
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
