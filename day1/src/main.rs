use std::fs;

fn task1(input: &str) -> i32 {
    let mut sum = 0;
    let mut hi = 0;

    for i in input.lines() {
        if i == "" {
            if sum > hi {
                hi = sum;
            }
            sum = 0;
            continue;
        }
        sum += i.parse::<i32>().unwrap_or(0);
    }
    return hi;
}

fn task2(input: &str) -> i32 {

    let mut sum = 0;
    let mut hi = [0, 0, 0];

    for i in input.lines() {
        if i == "" {
            if sum > hi[0] { 
                hi[2] = hi[1];
                hi[1] = hi[0];
                hi[0] = sum; 
            }
            else if sum > hi[1] { 
                hi[2] = hi[1];
                hi[1] = sum; 
            }
            else if sum > hi[2] { 
                hi[2] = sum; 
            }
            sum = 0;
            continue;
        }
        sum += i.parse::<i32>().unwrap_or(0);
    }
    return hi[0] + hi[1] + hi[2];
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
        assert_eq!(task1(&test), 24000);
    }
    #[test]
    fn test2() {
        let test = fs::read_to_string("test").expect("Couldn't read file");
        assert_eq!(task2(&test), 45000);
    }
}
