use regex::Regex;
use std::fs;

fn task1(input: &str) -> i32 {
    let mut sum = 0;
    let re = Regex::new(r",|-").unwrap();
    for i in input.lines() {
        let vec: Vec<i32> = re.split(i).map(|x| x.parse::<i32>().unwrap()).collect();
        if (vec[0] >= vec[2] && vec[1] <= vec[3]) || (vec[0] <= vec[2] && vec[1] >= vec[3]) {
            sum += 1;
        }
    }
    return sum;
}
fn task2(input: &str) -> i32 {
    let mut sum = 0;
    let re = Regex::new(r",|-").unwrap();
    for i in input.lines() {
        let vec: Vec<i32> = re.split(i).map(|x| x.parse::<i32>().unwrap()).collect();
        if (vec[0] >= vec[2] && vec[0] <= vec[3])
            || (vec[1] >= vec[2] && vec[1] <= vec[3])
            || (vec[2] >= vec[0] && vec[2] <= vec[1])
            || (vec[3] >= vec[0] && vec[3] <= vec[1])
        {
            sum += 1;
        }
    }
    return sum;
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
        assert_eq!(task1(&test), 2);
    }

    #[test]
    fn test2() {
        let test = fs::read_to_string("test").expect("Couldn't read file");
        assert_eq!(task2(&test), 4);
    }
}
