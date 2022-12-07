use std::fs;
use regex::Regex;

fn task1(input: &str, crates: Vec<Vec<char>>) -> String {

    let mut crates = crates;
    let re = Regex::new(r"\d+").unwrap();
    for line in input.lines() {
        let op: Vec<usize> = re.find_iter(line).map(|x| x.as_str().parse::<usize>().unwrap()).collect(); 
        for _ in 0..op[0] {
            let p = crates[op[1]-1].pop().unwrap();
            crates[op[2]-1].push(p);
        }
    }
    let mut rv = String::from("");
    for i in 0..crates.len() {
        rv.push(crates[i].pop().unwrap());
    }
    return rv
}

fn task2(input: &str, crates: Vec<Vec<char>>) -> String {
    
    let mut crates = crates;
    let re = Regex::new(r"\d+").unwrap();
    for line in input.lines() {
        let op: Vec<usize> = re.find_iter(line).map(|x| x.as_str().parse::<usize>().unwrap()).collect(); 
        let mut temp = vec!();
        for _ in 0..op[0] {
            let p = crates[op[1]-1].pop().unwrap();
            temp.push(p);
        }
        for i in temp.iter().rev() {
            crates[op[2]-1].push(*i);
        }
    }
    let mut rv = String::from("");
    for i in 0..crates.len() {
        rv.push(crates[i].pop().unwrap());
    }
    return rv
}

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");
    let crates = vec!(
        vec!('Q', 'M', 'G', 'C', 'L'), 
        vec!('R', 'D', 'L', 'C', 'T', 'F', 'H', 'G'),
        vec!('V', 'J', 'F', 'N', 'M', 'T', 'W', 'R'),
        vec!('J', 'F', 'D', 'V', 'Q', 'P'),
        vec!('N', 'F', 'M', 'S', 'L', 'B', 'T'),
        vec!('R', 'N', 'V', 'H', 'C', 'D', 'P'),
        vec!('H', 'C', 'T'),
        vec!('G', 'S', 'J', 'V', 'Z', 'N', 'H', 'P'),
        vec!('Z', 'F', 'H', 'G')
        );

    println!("{}", task1(&input, crates.clone()));
    println!("{}", task2(&input, crates.clone()));
}

#[cfg(test)]
mod test {
    use super::{task1, task2};
    use std::fs;

    #[test]
    fn test1() {
        let vec = vec!(
            vec!('Z', 'N'),
            vec!('M', 'C', 'D'),
            vec!('P')
            );
        let test = fs::read_to_string("test").expect("Couldn't read file");
        assert_eq!(task1(&test, vec), "CMZ");
    }

    #[test]
    fn test2() {
        let vec = vec!(
            vec!('Z', 'N'),
            vec!('M', 'C', 'D'),
            vec!('P')
            );
        
        let test = fs::read_to_string("test").expect("Couldn't read file");
        assert_eq!(task2(&test, vec), "MCD");
    }
}
