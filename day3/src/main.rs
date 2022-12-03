use std::collections::HashMap;
use std::fs;

fn task1(input: &str) -> i32 {
    let mut prio: HashMap<char, i32> = HashMap::new();
    for (i, j) in (b'a'..=b'z').chain(b'A'..=b'Z').enumerate() {
        prio.insert(j as char, i as i32 + 1);
    }

    let mut sum = 0;
    for i in input.lines() {
        let first = &i[0..i.len() / 2];
        let second = &i[i.len() / 2..i.len()];

        for j in first.chars() {
            if second.contains(j) {
                sum += prio[&j];
                break;
            }
        }
    }
    return sum;
}
fn task2(input: &str) -> i32 {
    let mut prio: HashMap<char, i32> = HashMap::new();
    for (i, j) in (b'a'..=b'z').chain(b'A'..=b'Z').enumerate() {
        prio.insert(j as char, i as i32 + 1);
    }

    let mut sum = 0;
    let mut input = input.lines();
    loop {
        let a = {
            match input.next() {
                Some(t) => t,
                None => {
                    break;
                }
            }
        };

        let b = input.next().unwrap();
        let c = input.next().unwrap();
        for j in a.chars() {
            if b.contains(j) && c.contains(j) {
                sum += prio[&j];
                break;
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
        let test = fs::read_to_string("test1").expect("Couldn't read file");
        assert_eq!(task1(&test), 157);
    }

    #[test]
    fn test2() {
        let test = fs::read_to_string("test2").expect("Couldn't read file");
        assert_eq!(task2(&test), 70);
    }
}
