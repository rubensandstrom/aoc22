use std::fs;

fn task1(input: &str) -> i32 {
    let mut sum = 0;
    for i in input.lines() {
        if let Some((key, val)) = i.split_once(' ') {
            //println!("{key} {val}");
            if key == "A" {
                if val == "X" {sum += 1+3}
                if val == "Y" {sum += 2+6}
                if val == "Z" {sum += 3+0}
            }
            if key == "B" {
                if val == "X" {sum += 1+0}
                if val == "Y" {sum += 2+3}
                if val == "Z" {sum += 3+6}
            }
            if key == "C" {
                if val == "X" {sum += 1+6}
                if val == "Y" {sum += 2+0}
                if val == "Z" {sum += 3+3}
            }
        }

    }
    return sum;
}
fn task2(input: &str) -> i32 {
    let mut sum = 0;
    for i in input.lines() {
        if let Some((key, val)) = i.split_once(' ') {
            //println!("{key} {val}");
            if key == "A" {
                if val == "X" {sum += 3+0}
                if val == "Y" {sum += 1+3}
                if val == "Z" {sum += 2+6}
            }
            if key == "B" {
                if val == "X" {sum += 1+0}
                if val == "Y" {sum += 2+3}
                if val == "Z" {sum += 3+6}
            }
            if key == "C" {
                if val == "X" {sum += 2+0}
                if val == "Y" {sum += 3+3}
                if val == "Z" {sum += 1+6}
            }
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
        assert_eq!(task1(&test), 15);
    }

    #[test]
    fn test2() {
        let test = fs::read_to_string("test").expect("Couldn't read file");
        assert_eq!(task2(&test), 12);
    }
}
