use std::{fs, collections::HashMap, usize};

type Coord = (usize, usize);

fn parse ( input: &str ) -> (Vec<Vec<i32>>, Coord, Coord) {

    let input = fs::read_to_string(input).expect("Couldn't read file!");

    
    let mut start: Coord = (0, 0);
    let mut goal: Coord = (0, 0);
    

    let mut parsed_input = vec![];
    for (row, l) in input.lines().enumerate() {
        let mut v = vec![];
        for (col, c) in l.chars().enumerate() {
            if c == 'S' { start = (row, col); }
            if c == 'E' { goal = (row, col); }
            v.push(c as i32 - 'a' as i32);
        }
        parsed_input.push(v);
    }
    return (parsed_input, start, goal);
}

fn task1 ( input: Vec<Vec<i32>>, start: Coord, goal: Coord ) -> i32 {

    let mut visited: HashMap<Coord, Vec<Coord>> = HashMap::new();
    let mut to_visit = vec![]; // might change to que if grows to big

    visited.insert(start, vec![]);
    to_visit.push(start);

    while !to_visit.is_empty() {
        let (row, col) = to_visit.remove(0); 
        let path = visited[&(row, col)].clone();

        println!("{:?}", to_visit);
        if row > 0 {
            if !visited.contains_key(&(row-1, col)) {
                if (row-1, col) == goal { return [path.clone(), vec![(row-1, col)]].concat().len() as i32; }
                else if (row, col) == start || (-1..=1).contains(&(input[row][col] - input[row-1][col])) {

                    visited.insert((row-1, col), [path.clone(), vec![(row-1, col)]].concat());
                    to_visit.push((row-1, col));
                }
            }
        }
        if col > 0 {
            if !visited.contains_key(&(row, col-1)) {
                if (row, col-1) == goal { return [path.clone(), vec![(row, col-1)]].concat().len() as i32; }
                else if (row, col) == start || (-1..=1).contains(&(input[row][col] - input[row][col-1])) {

                    visited.insert((row, col-1), [path.clone(), vec![(row, col-1)]].concat());
                    to_visit.push((row, col-1));
                }
            }
        }
        if row < input.len()-1 {
            if !visited.contains_key(&(row+1, col)) {
                if (row+1, col) == goal { return [path.clone(), vec![(row+1, col)]].concat().len() as i32; }
                else if (row, col) == start || (-1..=1).contains(&(input[row][col] - input[row+1][col])) {

                    visited.insert((row+1, col), [path.clone(), vec![(row+1, col)]].concat());
                    to_visit.push((row+1, col));
                }
            }
        }
        if col < input[0].len()-1 {
            if !visited.contains_key(&(row, col+1)) {
                if (row, col+1) == goal { return [path.clone(), vec![(row, col+1)]].concat().len() as i32; }
                else if (row, col) == start || (-1..=1).contains(&(input[row][col] - input[row][col+1])) {

                    visited.insert((row, col+1), [path.clone(), vec![(row, col+1)]].concat());
                    to_visit.push((row, col+1));
                }
            }
        }

    }
    println!("Couldn't find path!");
    return 0;
}

fn task2(input: &str) -> i32 {

    return 0;
}

fn main() {

    let (input, start, goal) = parse("input");
    println!( "start: {:?}", start );
    println!( "goal: {:?}", goal );

    println!("{}", task1(input, start, goal));

}

#[cfg(test)]
mod test {
    use super::{parse, task1, task2};
    use std::fs;

    #[test]
    fn test1() {
        let ( test, start, goal ) = parse("test"); 
        println!("{:?}", test);
        println!("{:?}", start);
        println!("{:?}", goal);
        assert_eq!(task1(test, start, goal), 31);
    }
    #[test]
    fn test2() {
        let test = fs::read_to_string("test").expect("Couldn't read file!");
        assert_eq!(task2(&test), 0);
    }
}
