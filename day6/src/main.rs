use std::fs;
use fancy_regex::Regex;

fn task1(input: &str) -> i32 {
    let re = Regex::new(r"(.).*\1+").unwrap();
    for i in 4..input.len() {
        if !re.is_match(&input[i-4..i]).unwrap() {
            return i as i32;
        }
    }
    return 0;
}

fn task2(input: &str) -> i32 {
    let re = Regex::new(r"(.).*\1+").unwrap();
    for i in 14..input.len() {
        if !re.is_match(&input[i-14..i]).unwrap() {
            return i as i32;
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

    #[test]
    fn test1() {
        assert_eq!(task1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(task1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(task1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(task1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(task1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test2() {
        assert_eq!(task2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(task2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(task2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(task2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(task2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
