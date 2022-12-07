use std::fs;

fn sorted_filesystem(input: &str) -> Vec<i32> {
    let mut filesystem: Vec<(&str, i32)> = vec!();
    let mut directory = ("/", 0);
    let mut temp = 0;
    let mut result: Vec<i32> = vec!(); 

    for line in input.lines() {
        if line == "$ cd .." {
            temp = directory.1;
            directory = filesystem.pop().unwrap();
            result.push(temp);
            directory.1 += temp;
        } else if &line[0..=3] == "$ cd" {
            filesystem.push(directory);
            directory = (&line.split(" ").nth(2).unwrap(), 0);
        } else {
            let memory: i32 = line.split(" ").nth(0).unwrap().parse().unwrap_or(0);
            directory.1 += memory;
        }
    }

    loop {
        temp = directory.1;
        let a = filesystem.pop();
        match a {
            Some(dir) => { 
                directory = dir;
                result.push(temp);
                directory.1 += temp; 
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
    let mut result = sorted_filesystem(input);

    let max = result.pop().unwrap();
    let req = 70_000_000 - max;
    for i in result {
        if i >= 30_000_000 - req {
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
