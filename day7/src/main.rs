use std::fs;

fn sorted_filesystem(input: &str) -> Vec<i32> {
    let mut filesystem: Vec<i32> = vec!();
    let mut directory = 0;
    let mut result: Vec<i32> = vec!(); 

    for line in input.lines() {
        if line == "$ cd .." {
            result.push(directory);
            directory += filesystem.pop().unwrap();
        } else if &line[0..=3] == "$ cd" {
            filesystem.push(directory);
            directory = 0;
        } else {
            let memory: i32 = line.split(" ").nth(0).unwrap().parse().unwrap_or(0);
            directory += memory;
        }
    }

    loop {
        let a = filesystem.pop();
        match a {
            Some(dir) => { 
                result.push(directory);
                directory += dir;
            }
            None => { break; }
        }
    }
    result.sort();
    return result;
}

fn task1(input: &str) -> i32 {

    let result = sorted_filesystem(input);
    let mut sum = 0;
    
    for i in result {
        if i > 100000 { break; }
        sum += i;
    }
    return sum;
}

fn task2(input: &str) -> i32 {

    let result = sorted_filesystem(input);

    let max = result.last().unwrap();
    let req = 30_000_000 - (70_000_000 - max);

    for i in result {
        if i >= req {
            return i;
        }
    }
    return 0;
}

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");

    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

#[cfg(test)]
mod test {
    use super::{task1, task2};
    use std::fs;

    #[test]
    fn test1() {
        let test = fs::read_to_string("test").expect("Couldn't read file");
        assert_eq!(task1(&test), 95437);
    }

    #[test]
    fn test2() {
        let test = fs::read_to_string("test").expect("Couldn't read file");
        assert_eq!(task2(&test), 24933642);
    }
}
